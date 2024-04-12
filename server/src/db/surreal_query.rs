use std::collections::HashMap;
use surrealdb::{engine::local::Db, Surreal};

#[allow(dead_code)]
pub enum SetMode { Equal, Add, Subtract }

#[allow(dead_code)]
pub enum ReturnType { None, Diff, Before, After, Fields{ fields: Vec<String> } }

pub struct Builder {
    parts: Vec<String>,
    bindings: HashMap<String, serde_json::Value>,
    alias_counters: HashMap<String, u32>
}

#[allow(dead_code)]
impl Builder {
    pub fn new() -> Self { 
        Builder { parts: vec![], bindings: HashMap::new(), alias_counters: HashMap::new() }
    }

    pub fn create(mut self) -> Self { self.parts.push("CREATE".into()); self }
    pub fn update(mut self) -> Self { self.parts.push("UPDATE".into()); self }
    pub fn delete(mut self) -> Self { self.parts.push("DELETE".into()); self }
    pub fn select(mut self) -> Self { self.parts.push("SELECT".into()); self }

    pub fn begin_tx(mut self) -> Self { self.parts.push("BEGIN TRANSACTION".into()); self }
    pub fn end_tx(mut self) -> Self { self.parts.push("COMMIT TRANSACTION".into()); self }
    pub fn cancel_tx(mut self) -> Self { self.parts.push("CANCEL TRANSACTION".into()); self }

    pub fn all(mut self) -> Self { self.parts.push("*".into()); self}
    pub fn some(mut self, some: String) -> Self { self.parts.push(some); self }

    pub fn from(mut self) -> Self { self.parts.push("FROM".into()); self }

    pub fn thing(mut self, tb: &str, uid: &str) -> Self {
        let tb_alias = self.next_alias("tb");
        let uid_alias = self.next_alias("uid");
        self.parts.push(format!("type::thing(${}, ${})", tb_alias, uid_alias));
        self.bindings.insert(tb_alias, tb.into());
        self.bindings.insert(uid_alias, uid.into());
        self
    }
    pub fn table(mut self, tb: &str) -> Self {
        let tb_alias = self.next_alias("tb");
        self.parts.push(format!("type::table(${})", tb_alias));
        self.bindings.insert(tb_alias, tb.into());
        self
    }

    pub fn content(self, data: serde_json::Value) -> Self {
        self.data("CONTENT", data)
    }

    pub fn merge(self, data: serde_json::Value) -> Self {
        self.data("MERGE", data)
    }

    pub fn set(mut self, field: &str, value: serde_json::Value, mode: SetMode) -> Self {
        let statement = if self.parts.len() > 0 && self.parts.last().unwrap().starts_with("SET")
            { "," } else { "SET" };
        let value_alias = self.next_alias("value");
        self.parts.push(format!("{} {} {} ${}", statement, field, mode.as_str(), value_alias));
        self.bindings.insert(value_alias, value);
        self
    }

    pub fn equals(mut self, field: &str, value: serde_json::Value) -> Self {
        let statement = if self.parts.len() > 0 && self.parts.last().unwrap().starts_with("WHERE")
            { "AND" } else { "WHERE" };
        let value_alias = self.next_alias("value");
        self.parts.push(format!("{} {} = ${}", statement, field, value_alias));
        self.bindings.insert(value_alias, value);
        self
    }

    pub fn returns(mut self, return_type: ReturnType) -> Self {
        self.parts.push(format!("RETURN {}", return_type.as_str()));
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut result = String::new();
        let mut previous: Option<&str> = None;

        for current in self.parts.iter() {
            if let Some(_) = previous {
                if current.ends_with("TRANSACTION") || current.starts_with("CREATE") ||
                current.starts_with("UPDATE") || current.starts_with("DELETE") {
                    result.push_str(";\r\n");
                } else if !current.starts_with(',') {
                    result.push(' ');
                }
            }
            result.push_str(current);
            previous = Some(current);
        }
        result.push(';');
        result
    }

    pub fn to_final_string(&self) -> String {
        let query = self.to_query_string();
        self.bindings.iter().fold(query, |acc, (key, value)| {
            acc.replace(&format!("${}", key), &serde_json::to_string(value).unwrap())
        })
    }

    pub async fn exec(&self, db: &Surreal<Db>) -> Result<surrealdb::Response, surrealdb::Error> {
        let query = self.to_query_string();
        self.bindings.iter().fold(db.query(query), |acc, (key, value)| {
            acc.bind((key, value))
        }).await
    }

    fn data(mut self, statement: &str, data: serde_json::Value) -> Self {
        let data_alias = self.next_alias("data");
        self.parts.push(format!("{} ${}", statement, data_alias));
        self.bindings.insert(data_alias, data);
        self
    }

    fn next_alias(&mut self, base: &str) -> String {
        if self.alias_counters.contains_key(base) {
            let count = self.alias_counters.get_mut(base).unwrap();
            *count += 1;
            if *count == 1 {
                return base.into();
            }
            format!("{}{}", base, count)
        } else {
            self.alias_counters.insert(base.into(), 1);
            self.next_alias(base)
        }
    }
}

impl SetMode {
    fn as_str(&self) -> &'static str {
        match self {
            SetMode::Equal => "=",
            SetMode::Add => "+=",
            SetMode::Subtract => "-=",
        }
    }
}

impl ReturnType {
    fn as_str(&self) -> String {
        match self {
            ReturnType::None => "NONE".into(),
            ReturnType::Diff => "DIFF".into(),
            ReturnType::Before => "BEFORE".into(),
            ReturnType::After => "AFTER".into(),
            ReturnType::Fields{ fields } => {
                fields.join(", ")
            }
        }
    }
}
