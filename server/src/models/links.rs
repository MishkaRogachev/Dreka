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
pub enum ProtocolVersion {
    MavlinkV1,
    MavlinkV2
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum LinkProtocol {
    Mavlink { link_type: LinkType, protocol_version: ProtocolVersion },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub struct LinkDescription {
    pub protocol: LinkProtocol,
    pub enabed: bool,
    pub autoconnect: bool
}
