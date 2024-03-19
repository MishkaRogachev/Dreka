use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};

use crate::persistence::{repository, traits};
use crate::models::communication::{LinkId, LinkDescription, LinkStatus};

#[derive(Clone)]
pub struct Persistence {
    pub link_descriptions: Arc<dyn traits::IRepository<LinkDescription> + Send + Sync>,
    pub link_statuses: Arc<dyn traits::IRepository<LinkStatus> + Send + Sync>
}

impl Persistence {
    pub fn new(db: Surreal<Db>) -> Self {
        Self {
            link_descriptions: Arc::new(repository::Repository::new(db.clone(), "link_descriptions")),
            link_statuses: Arc::new(repository::Repository::new(db, "link_statuses"))
        }
    }

    pub async fn save_link(&self, link: &LinkDescription) -> anyhow::Result<LinkDescription> {
        if link.id.is_empty() {
            let link = self.link_descriptions.create(link).await?;
            self.link_statuses.create(&LinkStatus::default_for_id(&link.id)).await?;
            Ok(link)

        } else {
            let link = self.link_descriptions.update(link).await?;
            Ok(link)
        }
    }

    pub async fn delete_link(&self, link_id: &LinkId) -> anyhow::Result<()> {
        self.link_descriptions.delete(link_id).await?;
        self.link_statuses.delete(link_id).await?;
        Ok(())
    }

    pub async fn update_status(&self, status: &LinkStatus) -> anyhow::Result<LinkStatus> {
        self.link_statuses.update(status).await
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
