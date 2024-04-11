use super::dal::Dal;

use crate::models::events::ServerEvent;
use crate::models::communication::{LinkId, LinkDescription, LinkStatus};

const TB_LINK_DESCRIPTIONS: &str = "link_descriptions";
const TB_LINK_STATUSES: &str = "link_statuses";

impl Dal {
    pub async fn save_link(&self, link: LinkDescription) -> anyhow::Result<LinkDescription> {
        let link = if link.id.is_empty() {
            let new_link = self.dao.create(TB_LINK_DESCRIPTIONS, link).await?;
            self.dao.create(TB_LINK_STATUSES, LinkStatus::default_for_id(&new_link.id)).await?;
            new_link
        } else {
            self.dao.update(TB_LINK_DESCRIPTIONS, link).await?
        };

        self.bus.publish(ServerEvent::LinkUpserted{ link: link.clone() })?;
        Ok(link)
    }

    pub async fn delete_link(&self, link_id: &LinkId) -> anyhow::Result<()> {
        self.dao.delete(TB_LINK_STATUSES, link_id).await?;
        self.dao.delete(TB_LINK_DESCRIPTIONS, link_id).await?;

        self.bus.publish(ServerEvent::LinkRemoved { link_id: link_id.into() })?;
        Ok(())
    }

    pub async fn update_link_status(&self, status: LinkStatus) -> anyhow::Result<LinkStatus> {
        let status = self.dao.update(TB_LINK_STATUSES, status).await?;
        self.bus.publish(ServerEvent::LinkStatusUpdated { status: status.clone() })?;
        Ok(status)
    }

    pub async fn link(&self, link_id: &LinkId) -> anyhow::Result<LinkDescription> {
        self.dao.select_one(TB_LINK_DESCRIPTIONS, link_id).await
    }

    pub async fn all_links(&self) -> anyhow::Result<Vec<LinkDescription>> {
        self.dao.select_all(TB_LINK_DESCRIPTIONS).await
    }

    pub async fn link_status(&self, link_id: &LinkId) -> anyhow::Result<LinkStatus> {
        self.dao.select_one(TB_LINK_STATUSES, link_id).await
    }

    pub async fn all_links_statuses(&self) -> anyhow::Result<Vec<LinkStatus>> {
        self.dao.select_all(TB_LINK_STATUSES).await
    }
}
