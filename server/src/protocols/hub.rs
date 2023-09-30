use std::sync::Arc;

use crate::models::links;
use crate::db::persistence;
use crate::protocols::mavlink::mavlink_connection;

pub struct Hub {
    persistence: Arc<persistence::Persistence>
}

impl Hub {
    pub fn new(persistence: Arc<persistence::Persistence>) -> Self {
        return Hub { persistence };
    }

    pub async fn start(&self) -> std::io::Result<()> {
        // TODO: scan persistence layer for saved connections

        // TODO: move to migration
        let default_udp_mavlink: links::LinkDescription = links::LinkDescription {
            protocol: links::LinkProtocol::Mavlink {
                link_type: links::LinkType::Udp {
                    address: String::from("127.0.0.1"),
                    port: 14540
                },
                protocol_version: links::ProtocolVersion::MavlinkV2
            },
            enabed: true,
            autoconnect: true
        };
        return self.start_link(&default_udp_mavlink).await;
    }

    pub async fn start_link(&self, link: &links::LinkDescription) -> std::io::Result<()> {
        match &link.protocol {
            links::LinkProtocol::Mavlink { link_type, protocol_version } => {
                return mavlink_connection::connect(link_type, protocol_version).await;
            },
            // NOTE: other protocols should be handled here
        }
    }
}