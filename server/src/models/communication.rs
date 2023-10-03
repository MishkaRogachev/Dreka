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
pub struct LinkDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(skip)]
    pub id: Option<surrealdb::sql::Thing>,
    pub protocol: LinkProtocol,
    pub enabled: bool,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub struct LinkStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(skip)]
    pub id: Option<surrealdb::sql::Thing>,
    pub is_connected: bool,
}
