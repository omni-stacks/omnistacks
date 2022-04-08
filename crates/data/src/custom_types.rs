use diesel::{
    backend,
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
};
use std::fmt::Display;
use std::io::Write;

#[derive(Debug, SqlType, Clone, Copy)]
#[postgres(type_name = "NodeMessageType")]
pub struct NodeMessageType;

#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression)]
#[sql_type = "NodeMessageType"]
pub enum MessageType {
    NewBurnBlock,
    NewBlock,
    NewMempoolTx,
    DropMempoolTx,
    NewAttachments,
    NewMicroBlocks,
}

impl ToSql<NodeMessageType, Pg> for MessageType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            MessageType::NewBurnBlock => out.write_all(b"new_burn_block")?,
            MessageType::NewBlock => out.write_all(b"new_block")?,
            MessageType::NewMempoolTx => out.write_all(b"new_mempool_tx")?,
            MessageType::DropMempoolTx => out.write_all(b"drop_mempool_tx")?,
            MessageType::NewAttachments => out.write_all(b"new_attachments")?,
            MessageType::NewMicroBlocks => out.write_all(b"new_microblocks")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<NodeMessageType, Pg> for MessageType {
    fn from_sql(bytes: Option<&<Pg as backend::Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"new_burn_block" => Ok(MessageType::NewBurnBlock),
            b"new_block" => Ok(MessageType::NewBlock),
            b"new_mempool_tx" => Ok(MessageType::NewMempoolTx),
            b"drop_mempool_tx" => Ok(MessageType::DropMempoolTx),
            b"new_attachments" => Ok(MessageType::NewAttachments),
            b"new_microblocks" => Ok(MessageType::NewMicroBlocks),
            _ => Err("Unrecognized NodeMessageType enum variant".into()),
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MessageType::NewBurnBlock => f.write_str("new_burn_block"),
            MessageType::NewBlock => f.write_str("new_block"),
            MessageType::NewMempoolTx => f.write_str("new_mempool_tx"),
            MessageType::DropMempoolTx => f.write_str("drop_mempool_tx"),
            MessageType::NewAttachments => f.write_str("new_attachments"),
            MessageType::NewMicroBlocks => f.write_str("new_microblocks"),
        }
    }
}
