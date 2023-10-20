use std::sync::{Arc, Mutex};
use tokio::time;
use tokio_util::sync::CancellationToken;
use mavlink;

use crate::models::communication;
use crate::services::communication::traits;

const MAVLINK_POLL_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(5);
const RESET_STATS_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(1000);
const ONLINE_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(2000);

pub struct MavlinkConnection {
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
    pub fn new(link_type: &communication::LinkType, protocol: &communication::MavlinkProtocolVersion) -> MavlinkConnection {
        MavlinkConnection {
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
    async fn connect(&mut self) -> Result<bool, traits::ConnectionError> {
        if let Some(token) = &self.token {
            if !token.is_cancelled() {
                println!("MAVLink {:?}:{:?} is already connected", &self.mav_address, &self.mav_version);
                return Ok(false);
            }
        }

        println!("MAVLink connect to {:?}:{:?}", &self.mav_address, &self.mav_version);

        let connected = mavlink::connect::<mavlink::common::MavMessage>(&self.mav_address);
        if let Err(err) = connected {
            return Err(traits::ConnectionError::Io(err));
        }

        let mut mav_connection = connected.unwrap();
        mav_connection.set_protocol_version(self.mav_version);

        let mav = Arc::new(mav_connection);

        // Token to stop polling mavlink packets
        let token = CancellationToken::new();
        let cloned_token = token.clone();
        self.token = Some(token);

        let mut last_stats_reset = time::Instant::now();

        let internal = self.internal.clone();
        let cloned_mav = mav.clone();
        tokio::task::spawn(async move { loop {
            let now = time::Instant::now();
            if now.checked_duration_since(last_stats_reset) > Some(RESET_STATS_INTERVAL) {
                let mut lock = internal.lock().unwrap();
                lock.bytes_received_sec = lock.bytes_received_current;
                lock.bytes_sent_sec = lock.bytes_sent_current;
                lock.bytes_received_current = 0;
                lock.bytes_sent_current = 0;
                last_stats_reset = time::Instant::now();
            }

            match cloned_mav.recv() {
                Ok((header, msg)) => {
                    let mut lock = internal.lock().unwrap();
                    // Log last recv time and bytes
                    lock.last_recieved = now;
                    lock.bytes_received_current = lock.bytes_received_current + std::mem::size_of_val(&msg);

                    //println!("Got mavlink message: {:?}:{:?}", &header, &msg);
                    // TODO: read telemetry
                },
                Err(mavlink::error::MessageReadError::Io(err)) => {
                    if let std::io::ErrorKind::WouldBlock = err.kind() {
                        //no messages currently available to receive -- wait a while
                        tokio::time::sleep(MAVLINK_POLL_INTERVAL).await;
                        continue;
                    } else {
                        println!("Got mavlink error: {:?}", &err);
                        cloned_token.cancel();
                        break;
                    }
                },
                _ => {}
            }

            if cloned_token.is_cancelled() {
                return;
            }
        }});

        Ok(true)
    }

    async fn disconnect(&mut self) -> Result<bool, traits::ConnectionError> {
        if let Some(token) = &self.token {
            if !token.is_cancelled() {
                println!("MAVLink disconnect from {:?}:{:?}", &self.mav_address, &self.mav_version);
                token.cancel();
                self.token = None;
                return Ok(true);
            }
        }

        println!("MAVLink {:?}:{:?} is already connected", &self.mav_address, &self.mav_version);
        return Ok(false);
    }

    fn is_healthy(&self) -> bool {
        if let Some(token) = &self.token {
            if !token.is_cancelled() {
                return true;
            }
        }
        false
    }

    fn is_online(&self) -> bool {
        let last_recieved_time = self.internal.lock().unwrap().last_recieved;
        if time::Instant::now().checked_duration_since(last_recieved_time) < Some(ONLINE_INTERVAL) {
            return true;
        } else {
            return false;
        }
    }

    fn bytes_received(&self) -> usize {
        return self.internal.lock().unwrap().bytes_received_sec;
    }

    fn bytes_sent(&self) -> usize {
        return self.internal.lock().unwrap().bytes_sent_sec;
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