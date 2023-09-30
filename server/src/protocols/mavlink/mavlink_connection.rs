use crate::models::links;

use mavlink;

impl links::LinkType {
    pub fn to_mavlink(&self) -> String {
        match self {
            links::LinkType::Udp { address, port } => {
                return format!("udpout:{}:{}", address, port)
            },
            links::LinkType::Tcp { address, port } => {
                return format!("tcpout:{}:{}", address, port)
            },
            links::LinkType::Serial { port, baud_rate } => {
                return format!("serial:{}:{}", port, baud_rate)
            },
        }
    }
}

impl links::ProtocolVersion {
    pub fn to_mavlink(&self) -> mavlink::MavlinkVersion {
        match self {
            links::ProtocolVersion::MavlinkV1 => return mavlink::MavlinkVersion::V1,
            links::ProtocolVersion::MavlinkV2 => return mavlink::MavlinkVersion::V2,
        }
    }
}

pub async fn connect(link_type: &links::LinkType, protocol: &links::ProtocolVersion) -> std::io::Result<()> {
    println!("MAVLink connect: {:?}:{:?}", &link_type, &protocol);

    let mut connection = mavlink::connect::<mavlink::common::MavMessage>(&link_type.to_mavlink())?;

    connection.set_protocol_version(protocol.to_mavlink());

    Ok(())
}