use serde::{Deserialize, Serialize};

pub type LinkId = String;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum LinkType {
    Udp { address: String, port: u16 },
    Tcp { address: String, port: u16 },
    Serial { port: String, baud_rate: usize }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum MavlinkProtocolVersion {
    MavlinkV1,
    MavlinkV2
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum LinkProtocol {
    Mavlink { link_type: LinkType, protocol_version: MavlinkProtocolVersion },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde_with::skip_serializing_none]
pub struct LinkDescription {
    pub id: LinkId,
    pub protocol: LinkProtocol,
    pub name: String,
    pub autoconnect: bool
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct LinkStatus {
    pub id: LinkId,
    pub is_enabled: bool,
    pub is_connected: bool,
    pub is_online: bool,
    pub bytes_received: usize,
    pub bytes_sent: usize
}

impl LinkStatus {
    pub fn default_for_id(link_id: &str) -> Self {
        Self {
            id: link_id.into(),
            is_enabled: false,
            is_connected: false,
            is_online: false,
            bytes_received: 0,
            bytes_sent: 0
        }
    }
}
