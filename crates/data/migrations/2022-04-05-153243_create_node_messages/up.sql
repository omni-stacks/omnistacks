create table node_messages (
  id bigserial not null,
  created_at timestamptz not null default CURRENT_TIMESTAMP,
  message_type text not null,
  body jsonb not null default '{}' :: jsonb,
  constraint node_messages_id primary key (id)
);