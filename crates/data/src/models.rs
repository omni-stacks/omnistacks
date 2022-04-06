use chrono::{DateTime, Utc};

use crate::schema::*;

#[derive(Debug, Identifiable, Queryable)]
pub struct NodeMessage {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub message_type: String,
    pub body: serde_json::Value,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "node_messages"]
pub struct NewNodeMessage<'a> {
    pub message_type: &'a str,
    pub body: &'a serde_json::Value,
}
