use std::sync::Arc;
use tokio::{time, sync::Mutex};
use tokio_util::sync::CancellationToken;
use mavlink;

use crate::models::telemetry::VehicleTelemetry;
use crate::{registry::registry, models::communication};
use crate::services::communication::traits;

use super::{commands::CommandHandler, telemetry::TelemetryHandler, heartbeat::HeartbeatHandler, context::MavlinkContext};

const MAVLINK_POLL_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(5);
const RESET_STATS_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(1000);
const ONLINE_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(2000);

pub struct MavlinkConnection {
    registry: registry::Registry,
    telemetry_tx: flume::Sender<VehicleTelemetry>,
    telemetry_rx: flume::Receiver<VehicleTelemetry>,
    mav_address: String,
    mav_version: mavlink::MavlinkVersion,
    token: Option<CancellationToken>,
    internal: Arc<Mutex<MavlinkConnectionInternal>>
}

struct MavlinkConnectionInternal {
    last_recieved: time::Instant,
    bytes_received_sec: usize,
    bytes_sent_sec: usize,
    bytes_received_current: usize,
    bytes_sent_current: usize,
}

impl MavlinkConnection {
    pub fn new(
        registry: registry::Registry,
        telemetry_tx: flume::Sender<VehicleTelemetry>,
        telemetry_rx: flume::Receiver<VehicleTelemetry>,
        link_type: &communication::LinkType,
        protocol: &communication::MavlinkProtocolVersion
    ) -> Self {
        Self {
            registry,
            telemetry_tx,
            telemetry_rx,
            mav_address: link_type.to_mavlink(),
            mav_version: protocol.to_mavlink(),
            token: None,
            internal: Arc::new(Mutex::new(MavlinkConnectionInternal {
                last_recieved: time::Instant::now(),
                bytes_received_sec: 0,
                bytes_sent_sec: 0,
                bytes_received_current: 0,
                bytes_sent_current: 0
            }))
        }
    }
}

#[async_trait::async_trait]
impl traits::IConnection for MavlinkConnection {
    async fn connect(&mut self) -> anyhow::Result<bool> {
        if let Some(token) = &self.token {
            if !token.is_cancelled() {
                log::warn!("MAVLink {:?}:{:?} is already connected", &self.mav_address, &self.mav_version);
                return Ok(false);
            }
        }

        log::info!("MAVLink is going connect to {:?}:{:?}...", &self.mav_address, &self.mav_version);

        let mav_connection_result = mavlink::connect::<mavlink::common::MavMessage>(&self.mav_address);
        let mut mav_connection = match mav_connection_result {
            Ok(mav_connection) => mav_connection,
            Err(err) => {
                log::error!("MAVLink connection error: {:?}", &err);
                return Ok(false);
            }
        };
        log::info!("MAVLink connection established");
        mav_connection.set_protocol_version(self.mav_version);

        let mav = Arc::new(mav_connection);

        // Token to stop polling mavlink packets
        let token = CancellationToken::new();
        let cloned_token = token.clone();
        self.token = Some(token);

        let mut last_stats_reset = time::Instant::now();
        let mav_context = Arc::new(Mutex::new(MavlinkContext::new(
            self.registry.clone(),
            self.telemetry_tx.clone(),
            self.telemetry_rx.clone(),
        )));
        let internal = self.internal.clone();
        let cloned_mav = mav.clone();

        tokio::task::spawn(async move {
            let mut heartbeat_handler = HeartbeatHandler::new(mav_context.clone());
            let mut telemetry_handler = TelemetryHandler::new(mav_context.clone());
            let mut command_handler = CommandHandler::new(mav_context.clone());

            loop {
                let now = time::Instant::now();
                if now.checked_duration_since(last_stats_reset) > Some(RESET_STATS_INTERVAL) {
                    let mut lock = internal.lock().await;
                    lock.bytes_received_sec = lock.bytes_received_current;
                    lock.bytes_sent_sec = lock.bytes_sent_current;
                    lock.bytes_received_current = 0;
                    lock.bytes_sent_current = 0;
                    last_stats_reset = time::Instant::now();
                }

                // Parse incomming packets
                match cloned_mav.recv() {
                    Ok((header, msg)) => {
                        let mut lock = internal.lock().await;
                        // Log last recv time and bytes
                        lock.last_recieved = now;
                        lock.bytes_received_current = lock.bytes_received_current + std::mem::size_of_val(&msg);

                        heartbeat_handler.handle_message(&header, &msg).await;
                        telemetry_handler.handle_message(&header, &msg).await;
                        command_handler.handle_message(&header, &msg).await;
                    },
                    Err(mavlink::error::MessageReadError::Io(err)) => {
                        if let std::io::ErrorKind::WouldBlock = err.kind() {
                            //no messages currently available to receive -- wait a while
                            tokio::time::sleep(MAVLINK_POLL_INTERVAL).await;
                        } else {
                            cloned_token.cancel();
                            let mut lock = internal.lock().await;
                            lock.bytes_received_sec = 0;
                            lock.bytes_sent_sec = 0;
                            lock.bytes_received_current = 0;
                            lock.bytes_sent_current = 0;
                            log::error!("MAVLink got internal error: {:?}", &err);
                            break;
                        }
                    },
                    _ => {}
                }

                // Send commands
                for command in command_handler.process_commands().await {
                    match cloned_mav.send_default(&command) {
                        Ok(_) => {},
                        Err(error) => println!("Mavlink send error: {:?}", error),
                    }
                }

                if cloned_token.is_cancelled() {
                    return;
                }
            }
        });

        Ok(true)
    }

    async fn disconnect(&mut self) -> anyhow::Result<bool> {
        if let Some(token) = &self.token {
            if !token.is_cancelled() {
                log::info!("MAVLink disconnect from {:?}:{:?}", &self.mav_address, &self.mav_version);
                token.cancel();
                self.token = None;
                return Ok(true);
            }
        }

        log::warn!("MAVLink {:?}:{:?} is already connected", &self.mav_address, &self.mav_version);
        return Ok(false);
    }

    async fn is_connected(&self) -> bool {
        if let Some(token) = &self.token {
            if !token.is_cancelled() {
                return true;
            }
        }
        false
    }

    async fn is_online(&self) -> bool {
        let last_recieved_time = self.internal.lock().await.last_recieved;
        if time::Instant::now().checked_duration_since(last_recieved_time) < Some(ONLINE_INTERVAL) {
            return true;
        } else {
            return false;
        }
    }

    async fn bytes_received(&self) -> usize {
        return self.internal.lock().await.bytes_received_sec;
    }

    async fn bytes_sent(&self) -> usize {
        return self.internal.lock().await.bytes_sent_sec;
    }
}

impl Drop for MavlinkConnection {
    fn drop(&mut self) {
        if let Some(token) = &self.token {
            token.cancel();
        }
    }
}

impl communication::LinkType {
    pub fn to_mavlink(&self) -> String {
        match self {
            communication::LinkType::Udp { address, port } => {
                return format!("udpout:{}:{}", address, port)
            },
            communication::LinkType::Tcp { address, port } => {
                return format!("tcpout:{}:{}", address, port)
            },
            communication::LinkType::Serial { port, baud_rate } => {
                return format!("serial:{}:{}", port, baud_rate)
            },
        }
    }
}

impl communication::MavlinkProtocolVersion {
    pub fn to_mavlink(&self) -> mavlink::MavlinkVersion {
        match self {
            communication::MavlinkProtocolVersion::MavlinkV1 => return mavlink::MavlinkVersion::V1,
            communication::MavlinkProtocolVersion::MavlinkV2 => return mavlink::MavlinkVersion::V2,
        }
    }
}