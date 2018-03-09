use serde_json;
use chrono;

#[derive(Debug, Queryable)]
pub struct EventRecord {
    pub id: i32,
    pub name: Option<String>,
    pub data: Option<serde_json::Value>,
    pub timestamp: chrono::NaiveDateTime,
    pub audit_sk: i32,
}