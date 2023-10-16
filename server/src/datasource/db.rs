use surrealdb::{Surreal, engine::local::Mem};

#[derive(Clone)]
pub struct Repository {
    db: Surreal<surrealdb::engine::local::Db>
}

/// An error originating from the SurrealDB client library
#[derive(Debug)]
pub enum DbError {
    Surreal(surrealdb::Error),
    JSon(serde_json::Error),

    NoIdSpecified,
    IdIsEmpty,
    NoData
}

// TODO: err
fn extract_id(json: &mut serde_json::Value) -> Option<String> {
    if let Some(obj) = json.as_object_mut() {
        if let Some(id) = obj.remove("id") {
            match id.as_str() {
                Some(str) => return Some(str.to_owned()),
                None => return None,
            }
        }
    }
    None
}

// TODO: err
fn replace_surreal_id(json: &mut serde_json::Value) {
    if let Some(obj) = json.as_object_mut() {
        if let Some(id) = obj.remove("id") {
            if let Some(id) = id["id"]["String"].as_str() {
                obj.insert("id".into(), serde_json::json!(id));
            }
        }
    }
}

impl Repository {
    pub async fn new() -> Result<Self, DbError> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("dreka").use_db("dreka").await?;

        Ok(Repository { db })
    }

    fn parse_one(&self, mut response: surrealdb::Response) -> Result<serde_json::Value, DbError> {
        let json: Option<serde_json::Value> = response.take(0)?;
        if let Some(mut json) = json {
            replace_surreal_id(&mut json);
            return Ok(json);
        }
        Err(DbError::NoData)
    }

    async fn create_impl(&self, table: &str, data: &serde_json::Value, id: Option<String>) -> Result<serde_json::Value, DbError> {
        match id {
            Some(id) => {
                if id.is_empty() {
                    return Err(DbError::IdIsEmpty);
                }

                let response = self.db.query("CREATE type::thing($tb, $uid) CONTENT $data")
                    .bind(("tb", table))
                    .bind(("uid", &id))
                    .bind(("data", serde_json::json!(data)))
                    .await?;

                return self.parse_one(response);
            },
            None => {
                let json_data = serde_json::json!(data);
                let response = self.db.query("CREATE type::table($tb) CONTENT $data")
                    .bind(("tb", table))
                    .bind(("data", &json_data))
                    .await?;

                return self.parse_one(response);
            },
        }
    }

    async fn update_impl(&self, table: &str, data: &serde_json::Value, id: &str) -> Result<serde_json::Value, DbError> {
        let response = self.db.query("UPDATE type::thing($tb, $uid) CONTENT $data")
            .bind(("tb", table))
            .bind(("uid", &id))
            .bind(("data", serde_json::json!(data)))
            .await?;

        return self.parse_one(response);
    }

    pub async fn create<D>(&self, table: &str, data: &D) -> Result<D, DbError>
    where D: serde::ser::Serialize + Sized + for<'de> serde::Deserialize<'de> {
        let mut data = serde_json::to_value(data)?;
        let id = extract_id(&mut data);
        let data = self.create_impl(table, &data, id).await?;
        let result: D = serde_json::from_value(data)?;
        return Ok(result);
    }

    pub async fn update<D>(&self, table: &str, data: &D) -> Result<D, DbError>
    where D: serde::ser::Serialize + ?Sized + for<'de> serde::Deserialize<'de> {
        let mut data = serde_json::to_value(data)?;
        match extract_id(&mut data) {
            Some(id) => {
                let data = self.update_impl(table, &data, &id).await?;
                let result: D = serde_json::from_value(data)?;
                return Ok(result);
            },
            None => Err(DbError::NoIdSpecified)
        }
    }

    pub async fn create_or_update<D>(&self, table: &str, data: &D) -> Result<D, DbError>
    where D: serde::ser::Serialize + ?Sized + for<'de> serde::Deserialize<'de> {
        let mut data = serde_json::to_value(data)?;
        let id = extract_id(&mut data);
        match id {
            Some(id) => {
                let contains = self.contains(table, &id).await?;
                let data_back: serde_json::Value;
                if contains {
                    data_back = self.update_impl(table, &data, &id).await?;
                } else {
                    data_back = self.create_impl(table, &data, Some(id.to_owned())).await?;
                }
                let result: D = serde_json::from_value(data_back)?;
                return Ok(result);
            },
            None => {
                let data = self.create_impl(table, &data, None).await?;
                let result: D = serde_json::from_value(data)?;
                return Ok(result);
            }
        }
    }

    // TODO: patch/merge

    pub async fn read<D>(&self, table: &str, id: &str) -> Result<D, DbError>
    where D: for<'de> serde::Deserialize<'de> {
        let mut response = self.db.query("SELECT * FROM ONLY type::thing($tb, $uid)")
            .bind(("tb", table))
            .bind(("uid", id)).await?;

        let json: Option<serde_json::Value> = response.take(0)?;
        if let Some(mut json) = json {
            replace_surreal_id(&mut json);
            let data: D = serde_json::from_value(json)?;
            return Ok(data);
        }
        Err(DbError::NoData)
    }

    pub async fn read_all<D>(&self, table: &str) -> Result<Vec<D>, DbError>
    where D: for<'de> serde::Deserialize<'de> {
        let mut response = self.db.query("SELECT * FROM type::table($tb)")
            .bind(("tb", table))
            .await?;

        let jsons: Vec<serde_json::Value> = response.take(0)?;
        let mut datas: Vec<D> = Vec::new();
        for mut json in jsons {
            replace_surreal_id(&mut json);
            let data: D = serde_json::from_value(json)?;
            datas.push(data);
        }
        Ok(datas)
    }

    pub async fn remove(&self, table: &str, id: &str) -> Result<(), DbError> {
        let response = self.db.query("DELETE type::thing($tb, $uid)")
            .bind(("tb", table))
            .bind(("uid", id))
            .await?;

        response.check()?;
        Ok(())
    }

    pub async fn contains(&self, table: &str, id: &str) -> Result<bool, DbError> {
        let response = self.db.query("SELECT id FROM ONLY type::thing($tb, $uid)")
            .bind(("tb", table))
            .bind(("uid", id)).await?;

        return Ok(!response.check().is_err());
    }
}

impl From<surrealdb::Error> for DbError {
    fn from(err: surrealdb::Error) -> Self {
        DbError::Surreal(err)
    }
}

impl From<serde_json::Error> for DbError {
    fn from(err: serde_json::Error) -> Self {
        DbError::JSon(err)
    }
}
impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DbError::Surreal(err) => write!(f, "{}", err),
            DbError::JSon(err) => write!(f, "{}", err),
            DbError::NoIdSpecified => write!(f, "No id was specified"),
            DbError::IdIsEmpty => write!(f, "Id is empty"),
            DbError::NoData => write!(f, "No data")
        }
    }
}