use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};

use crate::persistence::{repository, traits};
use crate::models::events::ServerEvent;
use crate::models::communication::{LinkId, LinkDescription, LinkStatus};

use super::bus;

#[derive(Clone)]
pub struct Persistence {
    link_descriptions: Arc<dyn traits::IRepository<LinkDescription> + Send + Sync>,
    link_statuses: Arc<dyn traits::IRepository<LinkStatus> + Send + Sync>,
    bus: bus::EventBus<ServerEvent>
}

impl Persistence {
    pub fn new(db: Surreal<Db>, bus: bus::EventBus<ServerEvent>) -> Self {
        Self {
            link_descriptions: Arc::new(repository::Repository::new(db.clone(), "link_descriptions")),
            link_statuses: Arc::new(repository::Repository::new(db, "link_statuses")),
            bus
        }
    }

    pub async fn save_link(&self, link: &LinkDescription) -> anyhow::Result<LinkDescription> {
        let link = if link.id.is_empty() {
            let new_link = self.link_descriptions.create(link).await?;
            self.link_statuses.create(&LinkStatus::default_for_id(&new_link.id)).await?;
            new_link
        } else {
            self.link_descriptions.update(link).await?
        };

        self.bus.publish(ServerEvent::LinkUpdated{ link: link.clone() })?;
        Ok(link)
    }

    pub async fn delete_link(&self, link_id: &LinkId) -> anyhow::Result<()> {
        self.link_descriptions.delete(link_id).await?;
        self.link_statuses.delete(link_id).await?;

        self.bus.publish(ServerEvent::LinkRemoved { link_id: link_id.into() })?;
        Ok(())
    }

    pub async fn update_status(&self, status: &LinkStatus) -> anyhow::Result<LinkStatus> {
        let status = self.link_statuses.update(status).await?;
        self.bus.publish(ServerEvent::LinkStatusUpdated { status: status.clone() })?;
        Ok(status)
    }

    pub async fn link(&self, link_id: &LinkId) -> anyhow::Result<LinkDescription> {
        self.link_descriptions.read(link_id).await
    }

    pub async fn all_links(&self) -> anyhow::Result<Vec<LinkDescription>> {
        self.link_descriptions.read_all().await
    }

    pub async fn status(&self, link_id: &LinkId) -> anyhow::Result<LinkStatus> {
        self.link_statuses.read(link_id).await
    }

    pub async fn all_statuses(&self) -> anyhow::Result<Vec<LinkStatus>> {
        self.link_statuses.read_all().await
    }
}
