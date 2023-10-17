

use std::sync::Arc;
use std::io::{Error, ErrorKind};

use crate::datasource::db;
use crate::models::communication;

pub async fn check_and_create_links(repo: &Arc<db::Repository>) -> std::io::Result<()> {
    let response = repo.read_all::<communication::LinkDescription>("link_descriptions").await;
    if let Err(err) = response {
        return Err(Error::new(ErrorKind::Other, err.to_string()));
    }

    let links = response.unwrap();
    // TODO: check link ids
    if links.len() > 0 {
        return Ok(());
    }

    create_link(repo, &communication::LinkDescription {
        id: Some("default_udp_link".into()),
        name: "Default Mavlink UDP".into(),
        protocol: communication::LinkProtocol::Mavlink {
            link_type: communication::LinkType::Udp {
                address: String::from("127.0.0.1"),
                port: 14550
            },
            protocol_version: communication::MavlinkProtocolVersion::MavlinkV2
        },
        enabled: false
    }).await?;

    create_link(repo, &communication::LinkDescription {
        id: Some("default_tcp_link".into()),
        name: "Default Mavlink TCP".into(),
        protocol: communication::LinkProtocol::Mavlink {
            link_type: communication::LinkType::Tcp {
                address: String::from("127.0.0.1"),
                port: 5760
            },
            protocol_version: communication::MavlinkProtocolVersion::MavlinkV2
        },
        enabled: true
    }).await?;

    Ok(())
}

async fn create_link(repo: &Arc<db::Repository>, link: &communication::LinkDescription) -> std::io::Result<()> {
    let response = repo.create("link_descriptions", link).await;
    match response {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::new(ErrorKind::Other, err.to_string()))
    }
}
