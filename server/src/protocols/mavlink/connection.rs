use std::sync::Arc;
use tokio::time;
use tokio_util::sync::CancellationToken;
use mavlink;

use crate::models::communication;

const MAVLINK_POLL_INTERVAL: time::Duration = time::Duration::from_millis(100);

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
    token: Option<CancellationToken>
}

impl MavlinkConnection {
    pub fn new(link_type: &communication::LinkType, protocol: &communication::MavlinkProtocolVersion) -> MavlinkConnection {
        MavlinkConnection { mav_address: link_type.to_mavlink(), mav_version: protocol.to_mavlink(), token: None }
    }
}

#[async_trait::async_trait]
impl crate::protocols::common::Connection for MavlinkConnection {
    async fn connect(&mut self) -> std::io::Result<()> {
        if self.token.is_some() {
            println!("MAVLink {:?}:{:?} is already connected", &self.mav_address, &self.mav_version);
            return Ok(());
        }

        println!("MAVLink connect to {:?}:{:?}", &self.mav_address, &self.mav_version);

        let mut mav_connection = mavlink::connect::<mavlink::common::MavMessage>(&self.mav_address)?;
        mav_connection.set_protocol_version(self.mav_version);

        let mav = Arc::new(mav_connection);

        let cloned_mav = mav.clone();
        let listening_task = tokio::task::spawn(async move {
            loop {
                match cloned_mav.recv() {
                    Ok((header, msg)) => {
                        println!("Got mavlink message: {:?}:{:?}", &header, &msg);
                    },
                    Err(err) => {
                        println!("Got mavlink error: {:?}", &err);
                    }
                }
                tokio::time::sleep(MAVLINK_POLL_INTERVAL).await;
            }
        });

        let token = CancellationToken::new();
        let cloned_token = token.clone();
        self.token = Some(token);

        tokio::select! {
            _ = listening_task => {}
            _ = cloned_token.cancelled() => {}
        };

        Ok(())
    }

    async fn disconnect(&mut self) -> std::io::Result<()> {
        match &self.token {
            Some(token) => {
                println!("MAVLink disconnect from {:?}:{:?}", &self.mav_address, &self.mav_version);
                token.cancel();
                self.token = None;
                Ok(())
            },
            None => {
                println!("MAVLink {:?}:{:?} is already disconnected", &self.mav_address, &self.mav_version);
                Ok(())
            },
        }
    }

    fn is_connected(&self) -> bool { self.token.is_some() }
}

impl Drop for MavlinkConnection {
    fn drop(&mut self) {
        if let Some(token) = &self.token {
            token.cancel();
        }
    }
}