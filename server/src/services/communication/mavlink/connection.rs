use std::sync::Arc;
use tokio::{time, sync::Mutex};
use tokio_util::sync::CancellationToken;
use mavlink;

use crate::models::{events::{ClientEvent, ServerEvent}, communication};
use crate::{bus::bus, dal::dal};

use crate::services::communication::traits;

use super::handler::Handler;

const MAVLINK_POLL_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(5);
const RESET_STATS_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(1000);
const ONLINE_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(2000);

pub struct MavlinkConnection {
    dal: dal::Dal,
    server_bus: bus::EventBus::<ServerEvent>,
    client_bus: bus::EventBus::<ClientEvent>,
    mav_address: String,
    mav_version: mavlink::MavlinkVersion,
    command_id: Option<CancellationToken>,
    statistics: Arc<Mutex<MavlinkConnectionStatistics>>
}

struct MavlinkConnectionStatistics {
    last_recieved: time::Instant,
    bytes_received_sec: usize,
    bytes_sent_sec: usize,
    bytes_received_current: usize,
    bytes_sent_current: usize,
}

impl MavlinkConnection {
    pub fn new(
        dal: dal::Dal,
        server_bus: bus::EventBus::<ServerEvent>,
        client_bus: bus::EventBus::<ClientEvent>,
        link_type: &communication::LinkType,
        protocol: &communication::MavlinkProtocolVersion
    ) -> Self {
        Self {
            dal,
            server_bus,
            client_bus,
            mav_address: link_type.to_mavlink(),
            mav_version: protocol.to_mavlink(),
            command_id: None,
            statistics: Arc::new(Mutex::new(MavlinkConnectionStatistics {
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
        if let Some(command_id) = &self.command_id {
            if !command_id.is_cancelled() {
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

        // command_id to stop polling mavlink packets
        let command_id = CancellationToken::new();
        let cloned_command_id = command_id.clone();
        self.command_id = Some(command_id);

        let mut last_stats_reset = time::Instant::now();
        let statistics = self.statistics.clone();
        let cloned_mav = mav.clone();
        let mut handler = Handler::new(self.dal.clone(), self.server_bus.clone(), self.client_bus.subscribe());

        tokio::task::spawn(async move {
            loop {
                let now = time::Instant::now();
                if now.checked_duration_since(last_stats_reset) > Some(RESET_STATS_INTERVAL) {
                    let mut lock = statistics.lock().await;
                    lock.bytes_received_sec = lock.bytes_received_current;
                    lock.bytes_sent_sec = lock.bytes_sent_current;
                    lock.bytes_received_current = 0;
                    lock.bytes_sent_current = 0;
                    last_stats_reset = time::Instant::now();
                }

                // Parse incomming packets
                match cloned_mav.recv() {
                    Ok((header, msg)) => {
                        let mut lock = statistics.lock().await;
                        // Log last recv time and bytes
                        lock.last_recieved = now;
                        lock.bytes_received_current = lock.bytes_received_current + std::mem::size_of_val(&msg);

                        handler.handle_message(&header, &msg).await;
                    },
                    Err(mavlink::error::MessageReadError::Io(err)) => {
                        match err.kind() {
                            std::io::ErrorKind::WouldBlock => {
                                //no messages currently available to receive -- wait a while
                                tokio::time::sleep(MAVLINK_POLL_INTERVAL).await;
                            },
                            _ => {
                                cloned_command_id.cancel();
                                let mut lock = statistics.lock().await;
                                lock.bytes_received_sec = 0;
                                lock.bytes_sent_sec = 0;
                                lock.bytes_received_current = 0;
                                lock.bytes_sent_current = 0;
                                log::error!("MAVLink got internal error: {:?}", &err);
                                break;
                            }
                        }
                    },
                    _ => {}
                }

                // Send messages
                for command in handler.prepare_messages().await {
                    match cloned_mav.send_default(&command) {
                        Ok(_) => {},
                        Err(error) => println!("Mavlink send message error: {:?}", error),
                    }
                }

                if cloned_command_id.is_cancelled() {
                    return;
                }
            }
        });

        Ok(true)
    }

    async fn disconnect(&mut self) -> anyhow::Result<bool> {
        if let Some(command_id) = &self.command_id {
            if !command_id.is_cancelled() {
                log::info!("MAVLink disconnect from {:?}:{:?}", &self.mav_address, &self.mav_version);
                command_id.cancel();
                self.command_id = None;
                return Ok(true);
            }
        }

        log::warn!("MAVLink {:?}:{:?} is already connected", &self.mav_address, &self.mav_version);
        return Ok(false);
    }

    async fn is_connected(&self) -> bool {
        if let Some(command_id) = &self.command_id {
            if !command_id.is_cancelled() {
                return true;
            }
        }
        false
    }

    async fn is_online(&self) -> bool {
        let last_recieved_time = self.statistics.lock().await.last_recieved;
        if time::Instant::now().checked_duration_since(last_recieved_time) < Some(ONLINE_INTERVAL) {
            return true;
        } else {
            return false;
        }
    }

    async fn bytes_received(&self) -> usize {
        return self.statistics.lock().await.bytes_received_sec;
    }

    async fn bytes_sent(&self) -> usize {
        return self.statistics.lock().await.bytes_sent_sec;
    }
}

impl Drop for MavlinkConnection {
    fn drop(&mut self) {
        if let Some(command_id) = &self.command_id {
            command_id.cancel();
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