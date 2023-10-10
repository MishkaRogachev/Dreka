use std::{sync::{Arc, Mutex}, time::{Instant, Duration}};
use tokio_util::sync::CancellationToken;
use mavlink;

use crate::models::communication;

const MAVLINK_POLL_INTERVAL: Duration = Duration::from_millis(5);
const ONLINE_INTERVAL: Duration = Duration::from_millis(2000);

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

pub struct MavlinkConnection {
    mav_address: String,
    mav_version: mavlink::MavlinkVersion,
    token: Option<CancellationToken>,
    last_recieved: Arc<Mutex<Instant>>,
}

impl MavlinkConnection {
    pub fn new(link_type: &communication::LinkType, protocol: &communication::MavlinkProtocolVersion) -> MavlinkConnection {
        MavlinkConnection {
            mav_address: link_type.to_mavlink(),
            mav_version: protocol.to_mavlink(),
            token: None,
            last_recieved: Arc::new(Mutex::new(Instant::now()))
        }
    }
}

#[async_trait::async_trait]
impl crate::protocols::traits::IConnection for MavlinkConnection {
    async fn connect(&mut self) -> std::io::Result<()> {
        if let Some(token) = &self.token {
            if !token.is_cancelled() {
                println!("MAVLink {:?}:{:?} is already connected", &self.mav_address, &self.mav_version);
                return Ok(());
            }
        }

        println!("MAVLink connect to {:?}:{:?}", &self.mav_address, &self.mav_version);

        let mut mav_connection = mavlink::connect::<mavlink::common::MavMessage>(&self.mav_address)?;
        mav_connection.set_protocol_version(self.mav_version);

        let mav = Arc::new(mav_connection);

        let token = CancellationToken::new();
        let cloned_token = token.clone();
        self.token = Some(token);

        let last_recieved = self.last_recieved.clone();
        let cloned_mav = mav.clone();
        tokio::task::spawn(async move { loop {
            match cloned_mav.recv() {
                Ok((header, msg)) => {
                    let mut time = last_recieved.lock().unwrap();
                    *time = Instant::now();

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

        Ok(())
    }

    async fn disconnect(&mut self) -> std::io::Result<()> {
        if let Some(token) = &self.token {
            if !token.is_cancelled() {
                println!("MAVLink disconnect from {:?}:{:?}", &self.mav_address, &self.mav_version);
                token.cancel();
                self.token = None;
                return Ok(())
            }
        }

        println!("MAVLink {:?}:{:?} is already connected", &self.mav_address, &self.mav_version);
        return Ok(());
    }

    fn is_connected(&self) -> bool {
        if let Some(token) = &self.token {
            if !token.is_cancelled() {
                return true;
            }
        }
        false
    }

    fn is_online(&self) -> bool {
        let last_recieved_time = self.last_recieved.lock().unwrap();
        if Instant::now().duration_since(*last_recieved_time) < ONLINE_INTERVAL {
            return true;
        } else {
            return false;
        }
    }
}

impl Drop for MavlinkConnection {
    fn drop(&mut self) {
        if let Some(token) = &self.token {
            token.cancel();
        }
    }
}