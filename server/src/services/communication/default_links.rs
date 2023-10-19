use std::sync::Arc;

use crate::datasource::db;
use crate::models::communication;

pub async fn create_dafault_links(repo: &Arc<db::Repository>) -> Result<Vec<communication::LinkDescription>, db::DbError> {
    let mut links = Vec::new();

    let link = create_link(repo, communication::LinkDescription {
        id: Some("default_udp_link".into()),
        name: "Default Mavlink UDP".into(),
        protocol: communication::LinkProtocol::Mavlink {
            link_type: communication::LinkType::Udp {
                address: String::from("127.0.0.1"),
                port: 14550
            },
            protocol_version: communication::MavlinkProtocolVersion::MavlinkV2
        },
        autoconnect: false
    }).await?;
    links.push(link);

    let link = create_link(repo, communication::LinkDescription {
        id: Some("default_tcp_link".into()),
        name: "Default Mavlink TCP".into(),
        protocol: communication::LinkProtocol::Mavlink {
            link_type: communication::LinkType::Tcp {
                address: String::from("127.0.0.1"),
                port: 5760
            },
            protocol_version: communication::MavlinkProtocolVersion::MavlinkV2
        },
        autoconnect: true
    }).await?;
    links.push(link);

    Ok(links)
}

async fn create_link(repo: &Arc<db::Repository>, link: communication::LinkDescription) -> Result<communication::LinkDescription, db::DbError> {
    repo.create("link_descriptions", &link).await?;
    Ok(link)
}
