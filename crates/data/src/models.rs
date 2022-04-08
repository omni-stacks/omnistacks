use crate::{custom_types::*, schema::*};
use chrono::{DateTime, Utc};

#[derive(Debug, Identifiable, Queryable)]
pub struct NodeMessage {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub message_type: MessageType,
    pub body: serde_json::Value,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "node_messages"]
pub struct NewNodeMessage<'a> {
    pub message_type: MessageType,
    pub body: &'a serde_json::Value,
}
