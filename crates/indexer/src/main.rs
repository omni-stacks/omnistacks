use std::time::Duration;
use tokio::time;

use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub use omnistacks_data::schema::node_messages::dsl::*;
use omnistacks_data::{custom_types::MessageType, db::*, db_pool, models::NodeMessage};

mod burn_block;

pub fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "DEBUG".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { run().await })
}

async fn run() {
    let db_pool = db_pool::get_pool().expect("Failed to get DB pool");

    const DELAY: f32 = 0.02;

    let mut last_processed_message_id = 0;

    loop {
        let conn = db_pool.get().expect("Failed to get DB connection");
        let results = node_messages
            .filter(id.gt(last_processed_message_id))
            .limit(10)
            .load::<NodeMessage>(&conn);
        match results {
            Ok(messages) => {
                for msg in &messages {
                    info!("Processing message [{}][{}]", msg.message_type, msg.id);

                    match &msg.message_type {
                        MessageType::NewBurnBlock => burn_block::process(&msg.body),
                        _ => {}
                    }

                    last_processed_message_id = msg.id;
                }

                if messages.is_empty() {
                    time::sleep(Duration::from_secs_f32(DELAY)).await;
                }
            }
            Err(e) => {
                error!("{}", e)
            }
        }
    }
}
