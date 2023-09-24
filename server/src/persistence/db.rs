
use std::sync::Arc;

use surrealdb::{kvs::Datastore, dbs::Session, sql::Value};

#[derive(Clone)]
pub struct Db {
    datastore: Arc<Datastore>,
    session: Session
}

impl Db {
    pub async fn new(path: &str) -> Result<Self, surrealdb::Error> {
        let datastore = Arc::new(Datastore::new(path).await?);
        let session = Session::default().with_db("dreka").with_ns("dreka");

        Ok(Db { datastore, session })
    }

    pub async fn exec(&self, statement: &str) -> Result<Vec<Value>, surrealdb::Error> {
        let responses = self.datastore.execute(statement, &self.session, None).await?;

        let mut result = Vec::new();
        for response in responses {
            result.push(response.result?.first())
        }
        Ok(result)
    }
}
