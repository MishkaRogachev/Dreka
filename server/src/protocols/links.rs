

use std::sync::Arc;
use std::io::{Error, ErrorKind};

use crate::db::persistence;
use crate::models::links;

pub async fn check_and_create_links(persistence: &Arc<persistence::Persistence>) -> std::io::Result<()> {
    let response = persistence.read_all::<links::LinkDescription>("links").await;
    if let Err(err) = response {
        return Err(Error::new(ErrorKind::Other, err.to_string()));
    }

    let links = response.unwrap();
    // TODO: check link ids
    if links.len() > 0 {
        return Ok(());
    }

    create_link(persistence, &links::LinkDescription {
        id: None,
        name: "Default Mavlink UDP".into(),
        protocol: links::LinkProtocol::Mavlink {
            link_type: links::LinkType::Udp {
                address: String::from("127.0.0.1"),
                port: 14540
            },
            protocol_version: links::MavlinkProtocolVersion::MavlinkV2
        },
        enabled: false
    }).await?;

    create_link(persistence, &links::LinkDescription {
        id: None,
        name: "Default Mavlink TCP".into(),
        protocol: links::LinkProtocol::Mavlink {
            link_type: links::LinkType::Tcp {
                address: String::from("127.0.0.1"),
                port: 5760
            },
            protocol_version: links::MavlinkProtocolVersion::MavlinkV2
        },
        enabled: true
    }).await?;

    Ok(())
}

async fn create_link(persistence: &Arc<persistence::Persistence>, link: &links::LinkDescription) -> std::io::Result<()> {
    let response = persistence.create("links", link).await;
    if let Err(err) = response {
        return Err(Error::new(ErrorKind::Other, err.to_string()));
    }

    println!("create_link: {:?}", link);

    Ok(())
}
