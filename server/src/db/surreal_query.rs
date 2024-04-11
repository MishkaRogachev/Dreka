use std::collections::HashMap;
use surrealdb::{engine::local::Db, Surreal};

pub struct Builder {
    parts: Vec<String>,
    bindings: HashMap<String, serde_json::Value>,
    alias_count: u32,
}

impl Builder {
    pub fn new() -> Self { 
        Builder { parts: vec![], bindings: HashMap::new(), alias_count: 0 } 
    }

    pub fn create(mut self) -> Self { self.parts.push("CREATE".into()); self }
    pub fn update(mut self) -> Self { self.parts.push("UPDATE".into()); self }
    pub fn delete(mut self) -> Self { self.parts.push("DELETE".into()); self }
    pub fn select(mut self) -> Self { self.parts.push("SELECT".into()); self }

    pub fn all(mut self) -> Self { self.parts.push("*".into()); self}
    #[allow(dead_code)]
    pub fn some(mut self, some: String) -> Self { self.parts.push(some); self }

    pub fn from(mut self) -> Self { self.parts.push("FROM".into()); self }

    pub fn thing(mut self, tb: &str, uid: &str) -> Self {
        self.parts.push("type::thing($tb, $uid)".into());
        self.bindings.insert("tb".into(), tb.into());
        self.bindings.insert("uid".into(), uid.into());
        self
    }
    pub fn table(mut self, tb: &str) -> Self {
        self.parts.push("type::table($tb)".into());
        self.bindings.insert("tb".into(), tb.into());
        self
    }

    pub fn content(mut self, data: serde_json::Value) -> Self {
        self.parts.push("CONTENT $data".into());
        self.bindings.insert("data".into(), data);
        self
    }

    pub fn equals(mut self, field: &str, value: serde_json::Value) -> Self {
        let first = if self.parts.len() > 0 && self.parts.last().unwrap().starts_with("WHERE")
            { "AND" } else { "WHERE" };
        let value_alias = self.next_alias();
        self.parts.push(format!("{} {} = ${}", first, field, value_alias));
        self.bindings.insert(value_alias, value);
        self
    }

    pub fn to_string(&self) -> String {
        self.parts.join(" ")
    }

    pub async fn exec(&self, db: &Surreal<Db>) -> Result<surrealdb::Response, surrealdb::Error> {
        let query = self.to_string();
        self.bindings.iter().fold(db.query(query), |acc, (key, value)| {
            acc.bind((key, value))
        }).await
    }

    fn next_alias(&mut self) -> String {
        self.alias_count += 1;
        if self.alias_count == 1 {
            return String::from("value");
        }
        format!("value{}", self.alias_count)
    }
}
