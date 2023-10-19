use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum LinkType {
    Udp { address: String, port: u16 },
    Tcp { address: String, port: u16 },
    Serial { port: String, baud_rate: usize }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum MavlinkProtocolVersion {
    MavlinkV1,
    MavlinkV2
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum LinkProtocol {
    Mavlink { link_type: LinkType, protocol_version: MavlinkProtocolVersion },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
#[serde_with::skip_serializing_none]
pub struct LinkDescription {
    pub id: Option<String>,
    pub protocol: LinkProtocol,
    pub name: String,
    pub autoconnect: bool
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub struct LinkStatus {
    pub id: String,
    pub is_connected: bool,
    pub is_online: bool
}
