create type NodeMessageType as enum (
    'new_burn_block',
    'new_block',
    'new_mempool_tx',
    'drop_mempool_tx',
    'new_attachments',
    'new_microblocks'
)
;

alter table node_messages 
alter column message_type type NodeMessageType using message_type::NodeMessageType
;