use std::collections::HashMap;
use tokio::time;

use crate::models::communication::{LinkId, LinkDescription, LinkStatus, LinkProtocol, LinkType, MavlinkProtocolVersion};
use crate::models::events::{ClientEvent, ServerEvent};
use crate::registry::bus;
use crate::registry::registry;
use super::{traits, mavlink::connection::MavlinkConnection};

type LinkConnection = Box<dyn traits::IConnection + Send + Sync>;
type LinkConnections = HashMap<LinkId, LinkConnection>;

const CHECK_CONNECTIONS_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(250);

pub struct Service {
    registry: registry::Registry,
    server_bus: bus::EventBus::<ServerEvent>,
    client_bus: bus::EventBus::<ClientEvent>,
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
    pub fn new(
        registry: registry::Registry,
        server_bus: bus::EventBus::<ServerEvent>,
        client_bus: bus::EventBus::<ClientEvent>
    ) -> Self {
        Self {
            registry,
            server_bus,
            client_bus,
            link_connections: LinkConnections::new()
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        // Load all links
        let links = self.load_links().await?;

        for link in links {
            // Invalidate statuses
            self.reset_link_status(&link.id).await?;

            // Autoconect specified ones
            if link.autoconnect {
                let result = self.enable_link(&link.id).await;
                if let Err(err) = result {
                    log::warn!("Autoconnect link error: {}", err);
                }
            }
        }

        let mut interval = time::interval(CHECK_CONNECTIONS_INTERVAL);

        let mut client_events_rx = self.client_bus.subscribe();
        loop {
            interval.tick().await;

            // Listen requests from client
            match client_events_rx.try_recv() {
                Ok(event) => {
                    let result = self.handle_client_event(event).await;
                    if let Err(err) = result {
                        log::error!("Handle event error: {}", err);
                    }
                },
                Err(err) => {
                    if err != tokio::sync::broadcast::error::TryRecvError::Empty {
                        log::error!("RX error: {}", err);
                    }
                }
            }

            for (link_id, connection) in self.link_connections.iter_mut() {
                let status = collect_connection_status(link_id, connection).await;
                if !status.is_connected {
                    if let Err(err) = connection.connect().await {
                        log::warn!("Connect link error: {}", err);
                    }
                }

                if let Err(err) = self.registry.communication.update_status(&status).await {
                    log::error!("Update status error: {}", err);
                }
            }
        }
    }

    async fn load_links(&self) -> anyhow::Result<Vec<LinkDescription>> {
        let mut links = self.registry.communication.all_links().await?;

        if links.is_empty() {
            for link in dafault_links().iter() {
                let link = self.registry.communication.save_link(link).await?;
                links.push(link);
            }
        }
        Ok(links)
    }

    async fn enable_link(&mut self, link_id: &LinkId) -> anyhow::Result<()> {
        if self.link_connections.contains_key(link_id) {
            log::warn!("Link is already enabled {}", link_id);
            return Ok(());
        }

        let link = self.registry.communication.link(link_id).await?;
        let mut connection = self.create_connection(&link)?;
        if let Err(err) = connection.connect().await {
            log::warn!("Error enabling link: {}", err);
        }

        let status = collect_connection_status(&link_id, &connection).await;
        self.registry.communication.update_status(&status).await?;
        self.link_connections.insert(link_id.to_string(), connection);
        Ok(())
    }

    fn create_connection(&self, link: &LinkDescription) -> anyhow::Result<LinkConnection> {
        match &link.protocol {
            LinkProtocol::Mavlink { link_type, protocol_version } => {
                Ok(Box::new(MavlinkConnection::new(
                    self.registry.clone(),
                    self.server_bus.clone(),
                    link_type,
                    protocol_version
                )))
            },
            // NOTE: other protocols should be handled here
        }
    }

    async fn disable_link(&mut self, link_id: &LinkId) -> anyhow::Result<()> {
        if !self.link_connections.contains_key(link_id) {
            log::warn!("No connection found for link {}", link_id);
            return Ok(());
        }

        let mut connection = self.link_connections.remove(link_id).unwrap();
        connection.disconnect().await?;
        self.reset_link_status(link_id).await
    }

    async fn handle_client_event(&mut self, event: ClientEvent) -> anyhow::Result<()> {
        match event {
            ClientEvent::SetLinkEnabled { link_id, enabled: connected } => {
                if connected {
                    return self.enable_link(&link_id).await;
                }
                return self.disable_link(&link_id).await;
            },
            _ => Ok(())
        }
    }

    async fn reset_link_status(&self, link_id: &LinkId) -> anyhow::Result<()> {
        self.registry.communication.update_status(&LinkStatus::default_for_id(link_id)).await?;
        Ok(())
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
