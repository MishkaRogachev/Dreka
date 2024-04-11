use surrealdb::{engine::local::Db, Surreal};

use super::surreal_query::Builder;

const ID: &str = "id";
const STRING: &str = "String";

#[derive(Clone)]
pub struct Dao {
    db: Surreal<Db>
}

impl Dao {
    pub fn new(db: Surreal<Db>) -> Self {
        Self { db }
    }

    pub async fn create<T>(&self, table: &str, value: T) -> anyhow::Result<T>
    where T: serde::ser::Serialize + for<'de> serde::Deserialize<'de> {
        let mut data = serde_json::to_value(value)?;
        let mut query = Builder::new().create();
        query = match extract_surreal_id(&mut data) {
            Some(id) => query.thing(table, &id),
            None => query.table(table)
        };
        let response = query
            .content(data)
            .exec(&self.db).await?;
        parse_one_value(response)
    }

    pub async fn update<T>(&self, table: &str, value: T) -> anyhow::Result<T>
    where T: serde::ser::Serialize + for<'de> serde::Deserialize<'de> {
        let mut data = serde_json::to_value(value)?;
        if let Some(id) = extract_surreal_id(&mut data) {
            let response = Builder::new()
                .update()
                .thing(table, &id)
                .content(data)
                .exec(&self.db).await?;
            parse_one_value(response)
        } else {
            Err(anyhow::anyhow!("No id provided"))
        }
    }

    pub async fn delete(&self, table: &str, id: &str) -> anyhow::Result<()> {
        let response = Builder::new().delete().thing(table, id.into()).exec(&self.db).await?;
        response.check()?;
        Ok(())
    }

    pub async fn select_one<T>(&self, table: &str, id: &str) -> anyhow::Result<T>
    where T: for<'de> serde::Deserialize<'de> {
        let response = Builder::new().select().all().from().thing(table, id.into()).exec(&self.db).await?;
        parse_one_value(response)
    }

    pub async fn select_where<T, D>(&self, table: &str, field: &str, value: T) -> anyhow::Result<Vec<D>>
    where T: serde::ser::Serialize, D: for<'de> serde::Deserialize<'de> {
        let value = serde_json::to_value(value)?;
        let response = Builder::new().select().all().from().table(table)
            .equals(field, value).exec(&self.db).await?;
        parse_many_values(response)
    }

    pub async fn select_all<T>(&self, table: &str) -> anyhow::Result<Vec<T>>
    where T: for<'de> serde::Deserialize<'de> {
        let response = Builder::new().select().all().from().table(table).exec(&self.db).await?;
        parse_many_values(response)
    }
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

fn parse_one_value<T>(mut response: surrealdb::Response) -> anyhow::Result<T>
where T: for<'de> serde::Deserialize<'de> {
    let json: Option<serde_json::Value> = response.take(0)?;
    if let Some(mut json) = json {
        replace_surreal_id(&mut json);
        return Ok(serde_json::from_value(json)?);
    }
    Err(anyhow::anyhow!("No signle object found in response"))
}

fn parse_many_values<T>(mut response: surrealdb::Response) -> anyhow::Result<Vec<T>>
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
