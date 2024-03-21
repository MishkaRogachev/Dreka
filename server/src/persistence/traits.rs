use async_trait::async_trait;

#[async_trait]
pub trait IRepository<T> {
    async fn create(&self, entity: &T) -> anyhow::Result<T>;
    async fn read(&self, id: &str) -> anyhow::Result<T>;
    async fn read_all(&self) -> anyhow::Result<Vec<T>>;
    async fn read_where(&self, field: &str, value: serde_json::Value) -> anyhow::Result<Vec<T>>;
    async fn read_all_ids(&self) -> anyhow::Result<Vec<String>>;
    async fn update(&self, entity: &T) -> anyhow::Result<T>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
}
