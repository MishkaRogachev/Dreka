use surrealdb::{engine::local::Db, Surreal};

const ID: &str = "id";
const STRING: &str = "String";
const TB: &str = "tb";
const UID: &str = "uid";
const DATA: &str = "data";
const VALUE: &str = "value";
const FIELD: &str = "field";

const CREATE_THING_QUERY: &str = "CREATE type::thing($tb, $uid) CONTENT $data";
const CREATE_TABLE_QUERY: &str = "CREATE type::table($tb) CONTENT $data";
const SELECT_ONE_QUERY: &str = "SELECT * FROM type::thing($tb, $uid)";
const SELECT_ALL_QUERY: &str = "SELECT * FROM type::table($tb)";
const SELECT_ALL_IDS_QUERY: &str = "SELECT id FROM type::table($tb)";
const UPDATE_QUERY: &str = "UPDATE type::thing($tb, $uid) CONTENT $data";
const DELETE_QUERY: &str = "DELETE type::thing($tb, $uid)";

#[derive(Clone)]
pub struct Repository {
    db: Surreal<Db>,
    table: String
}

fn replace_surreal_id(json: &mut serde_json::Value) {
    if let Some(obj) = json.as_object_mut() {
        if let Some(surreal_id) = obj.remove(ID) {
            if let Some(id) = surreal_id[ID][STRING].as_str() {
                obj.insert(ID.into(), serde_json::json!(id));
            }
        }
    }
}

fn extract_surreal_id(json: &mut serde_json::Value) -> Option<String> {
    if let Some(obj) = json.as_object_mut() {
        if let Some(id) = obj.remove(ID) {
            if let Some(str) = id.as_str() {
                if str.is_empty() {
                    return None;
                }
                return Some(str.to_owned());
            }
        }
    }
    None
}

impl Repository {
    pub fn new(db: Surreal<Db>, table: &str) -> Self {
        Self { db, table: table.to_string() }
    }

    fn parse_one_json<T>(&self, mut response: surrealdb::Response) -> anyhow::Result<T>
    where T: for<'de> serde::Deserialize<'de> {
        let json: Option<serde_json::Value> = response.take(0)?;
        if let Some(mut json) = json {
            replace_surreal_id(&mut json);
            return Ok(serde_json::from_value(json)?);
        }
        Err(anyhow::anyhow!("Empty response provided"))
    }

    fn parse_many_json<T>(&self, mut response: surrealdb::Response) -> anyhow::Result<Vec<T>>
    where T: for<'de> serde::Deserialize<'de> {
        let mut jsons: Vec<serde_json::Value> = response.take(0)?;
        if jsons.is_empty() {
            return Ok(Vec::new());
        }
        let mut result: Vec<T> = Vec::with_capacity(jsons.len());
        for json in &mut jsons {
            replace_surreal_id(json);
            let item: T = serde_json::from_value(json.to_owned())?;
            result.push(item);
        }
        Ok(result)
    }
}

#[async_trait::async_trait]
impl<T> super::traits::IRepository<T> for Repository
where T: serde::ser::Serialize + ?Sized + for<'de> serde::Deserialize<'de> + std::marker::Sync + Clone {
    async fn create(&self, entity: &T) -> anyhow::Result<T> {
        let mut data = serde_json::to_value(entity)?;
        let id = extract_surreal_id(&mut data);
        match id {
            Some(id) => {
                let response = self.db.query(CREATE_THING_QUERY)
                    .bind((TB, &self.table))
                    .bind((UID, &id))
                    .bind((DATA, data))
                    .await?;
                self.parse_one_json(response)
            },
            None => {
                let response = self.db.query(CREATE_TABLE_QUERY)
                    .bind((TB, &self.table))
                    .bind((DATA, data))
                    .await?;
                self.parse_one_json(response)
            },
        }
    }

    async fn read(&self, id: &str) -> anyhow::Result<T> {
        let response = self.db.query(SELECT_ONE_QUERY)
            .bind((TB, &self.table))
            .bind((UID, id)).await?;
        self.parse_one_json(response)
    }

    async fn read_all(&self) -> anyhow::Result<Vec<T>> {
        let response = self.db.query(SELECT_ALL_QUERY)
            .bind((TB, &self.table)).await?;
        self.parse_many_json(response)
    }

    async fn read_where(&self, field: &str, value: serde_json::Value) -> anyhow::Result<Vec<T>> {
        let query = format!("SELECT * FROM type::table($tb) WHERE {} = $value", field);
        let response = self.db.query(query)
            .bind((TB, &self.table))
            .bind((FIELD, field))
            .bind((VALUE, value)).await?;
        self.parse_many_json(response)
    }

    async fn read_all_ids(&self) -> anyhow::Result<Vec<String>> {
        let mut response = self.db.query(SELECT_ALL_IDS_QUERY)
            .bind((TB, &self.table)).await?;
        let jsons: Vec<serde_json::Value> = response.take(0)?;
        let ids = jsons.into_iter().filter_map(|json| {
            println!("{:?}", json);
            if let Some(id) = json[ID][ID][STRING].as_str() {
                Some(id.to_owned())
            } else {
                None
            }
        }).collect();
        Ok(ids)
    }

    async fn update(&self, entity: &T) -> anyhow::Result<T> {
        let mut data = serde_json::to_value(entity)?;
        let id = extract_surreal_id(&mut data);
        match id {
            Some(id) => {
                let response = self.db.query(UPDATE_QUERY)
                    .bind((TB, &self.table))
                    .bind((UID, &id))
                    .bind((DATA, data))
                    .await?;
                self.parse_one_json(response)
            },
            None => {
                Err(anyhow::anyhow!("No id provided"))
            },
        }
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        let response = self.db.query(DELETE_QUERY)
            .bind((TB, &self.table))
            .bind((UID, id))
            .await?;
        response.check()?;
        Ok(())
    }
}
