table! {
    node_messages (id) {
        id -> Int8,
        created_at -> Timestamptz,
        message_type -> Text,
        body -> Jsonb,
    }
}
