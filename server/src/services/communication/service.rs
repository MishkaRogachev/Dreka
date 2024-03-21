use std::collections::HashMap;
use tokio::{sync::broadcast, time};

use crate::context::AppContext;
use crate::models::communication::{LinkId, LinkDescription, LinkStatus, LinkProtocol, LinkType, MavlinkProtocolVersion};
use crate::models::events::ClentEvent;
use super::{traits, mavlink::connection::MavlinkConnection};

type LinkConnection = Box<dyn traits::IConnection + Send + Sync>;
type LinkConnections = HashMap<LinkId, LinkConnection>;

const CHECK_CONNECTIONS_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(250);

pub struct Service {
    context: AppContext,
    rx: broadcast::Receiver<ClentEvent>,
    link_connections: LinkConnections
}

fn dafault_links() -> Vec<LinkDescription> {
    vec!(LinkDescription {
        id: "default_udp_link".into(),
        name: "Default Mavlink UDP".into(),
        protocol: LinkProtocol::Mavlink {
            link_type: LinkType::Udp {
                address: String::from("127.0.0.1"),
                port: 14550
            },
            protocol_version: MavlinkProtocolVersion::MavlinkV2
        },
        autoconnect: false
    },
    LinkDescription {
        id: "default_tcp_link".into(),
        name: "Default Mavlink TCP".into(),
        protocol: LinkProtocol::Mavlink {
            link_type: LinkType::Tcp {
                address: String::from("127.0.0.1"),
                port: 5760
            },
            protocol_version: MavlinkProtocolVersion::MavlinkV2
        },
        autoconnect: true
    })
}

impl Service {
    pub fn new(context: AppContext, rx: broadcast::Receiver<ClentEvent>) -> Self {
        Self { context, rx, link_connections: LinkConnections::new() }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        // Load all links
        let links = self.load_links().await?;

        for link in links {
            // Invalidate statuses
            self.context.communication.update_status(&LinkStatus::default_for_id(&link.id)).await?;

            // Autoconect specified ones
            if link.autoconnect {
                let result = self.enable_link(&link.id).await;
                if let Err(err) = result {
                    println!("Autoconnect link error: {}", err);
                }
            }
        }

        let mut interval = time::interval(CHECK_CONNECTIONS_INTERVAL);
        interval.tick().await;  // skip first tick

        loop { // NOTE: NO await? in this loop, use logger!
            interval.tick().await;

            // Listen requests from client
            match self.rx.try_recv() {
                Ok(event) => {
                    let result = self.handle_client_event(event).await;
                    if let Err(err) = result {
                        println!("Handle event error: {}", err);
                    }
                },
                Err(err) => {
                    if err != tokio::sync::broadcast::error::TryRecvError::Empty {
                        println!("RX error: {}", err);
                    }
                }
            }

            for (link_id, connection) in self.link_connections.iter_mut() {
                let status = collect_connection_status(link_id, connection).await;
                if !status.is_connected {
                    if let Err(err) = connection.connect().await {
                        println!("Connect link error: {}", err);
                    }
                }

                if let Err(err) = self.context.communication.update_status(&status).await {
                    println!("Update status error: {}", err);
                }
            }
        }
    }

    async fn load_links(&self) -> anyhow::Result<Vec<LinkDescription>> {
        let mut links = self.context.communication.all_links().await?;

        if links.is_empty() {
            for link in dafault_links().iter() {
                let link = self.context.communication.save_link(link).await?;
                links.push(link);
            }
        }
        Ok(links)
    }

    async fn enable_link(&mut self, link_id: &LinkId) -> anyhow::Result<()> {
        if self.link_connections.contains_key(link_id) {
            println!("Link is already enabled {}", link_id);
            return Ok(());
        }

        let link = self.context.communication.link(link_id).await?;
        let mut connection = create_connection(self.context.clone(), &link)?;
        if let Err(err) = connection.connect().await {
            println!("Error enabling link: {}", err);
        }

        let status = collect_connection_status(&link_id, &connection).await;
        self.context.communication.update_status(&status).await?;
        self.link_connections.insert(link_id.to_string(), connection);
        Ok(())
    }

    async fn disable_link(&mut self, link_id: &str) -> anyhow::Result<()> {
        if !self.link_connections.contains_key(link_id) {
            println!("No connection found for link {}", link_id);
            return Ok(());
        }

        let mut connection = self.link_connections.remove(link_id).unwrap();
        connection.disconnect().await?;
        self.context.communication.update_status(&LinkStatus::default_for_id(&link_id)).await?;
        Ok(())
    }

    async fn handle_client_event(&mut self, event: ClentEvent) -> anyhow::Result<()> {
        match event {
            ClentEvent::SetLinkEnabled { link_id, connected } => {
                if connected {
                    return self.enable_link(&link_id).await;
                } else {
                    return self.disable_link(&link_id).await;
                }
            }
        }
    }
}

fn create_connection(context: AppContext, link: &LinkDescription) -> anyhow::Result<LinkConnection> {
    match &link.protocol {
        LinkProtocol::Mavlink { link_type, protocol_version } => {
            Ok(Box::new(MavlinkConnection::new(context, link_type, protocol_version)))
        },
        // NOTE: other protocols should be handled here
    }
}

async fn collect_connection_status(link_id: &str, connection: &LinkConnection) -> LinkStatus {
    LinkStatus {
        id: link_id.into(),
        is_enabled: true,
        is_connected: connection.is_connected().await,
        is_online: connection.is_online().await,
        bytes_received: connection.bytes_received().await,
        bytes_sent: connection.bytes_sent().await,
    }
}
