use omnistacks_data::node_messages::NewBurnBlockMessage;
use serde_json::Value;
use tracing::error;

pub fn process(msg: &Value) {
    let parsed_msg = serde_json::from_value::<NewBurnBlockMessage>(msg.clone());
    match parsed_msg {
        Ok(_burn_block_msg) => {}
        Err(e) => error!("Failed to parse NewBurnBlock message: {}", e),
    }
}
