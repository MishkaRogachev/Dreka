
use surrealdb::{Surreal, engine::local::Mem};

#[derive(Clone)]
pub struct Persistence {
    db: Surreal<surrealdb::engine::local::Db>
}

impl Persistence {
    pub async fn new() -> Result<Self, surrealdb::Error> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("dreka").use_db("dreka").await?;

        Ok(Persistence { db })
    }

    pub async fn create<D>(&self, table: &str, id: &str, data: &D) -> Result<D, surrealdb::Error> 
    where D: serde::ser::Serialize + for<'de> serde::Deserialize<'de> {
        let record: Option<D> = self.db.create((table, id)).content(data).await?;
        Ok(record.unwrap())
    }

    pub async fn update<D>(&self, table: &str, id: &str, data: &D) -> Result<D, surrealdb::Error> 
    where D: serde::ser::Serialize + for<'de> serde::Deserialize<'de> {
        let record: Option<D> = self.db.update((table, id)).merge(data).await?;
        Ok(record.unwrap())
    }

    pub async fn read<D>(&self, table: &str, id: &str) -> Result<Option<D>, surrealdb::Error> 
    where D: serde::ser::Serialize + for<'de> serde::Deserialize<'de> {
        let record: Option<D> = self.db.select((table, id)).await?;
        Ok(record)
    }

    #[allow(dead_code)]
    pub async fn read_all<D>(&self, table: &str) -> Result<Vec<D>, surrealdb::Error> 
    where D: serde::ser::Serialize + for<'de> serde::Deserialize<'de> {
        let records: Vec<D> = self.db.select(table).await?;
        Ok(records)
    }

    pub async fn remove<D>(&self, table: &str, id: &str) -> Result<D, surrealdb::Error> 
    where D: serde::ser::Serialize + for<'de> serde::Deserialize<'de> {
        let record: Option<D> = self.db.delete((table, id)).await?;
        Ok(record.unwrap())
    }
}
