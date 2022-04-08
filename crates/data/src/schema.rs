table! {
    use diesel::sql_types::*;
    use crate::custom_types::NodeMessageType;

    node_messages (id) {
        id -> Int8,
        created_at -> Timestamptz,
        message_type -> NodeMessageType,
        body -> Jsonb,
    }
}
