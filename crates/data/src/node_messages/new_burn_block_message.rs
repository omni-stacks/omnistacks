use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewBurnBlockMessage {
    pub burn_block_hash: String,
    pub burn_block_height: i64,
    pub burn_amount: i64,
    pub reward_recipients: Vec<RewardRecipient>,
    pub reward_slot_holders: Vec<RewardSlotHolder>,
}

#[derive(Debug, Deserialize)]
pub struct RewardRecipient {
    pub recipient: String,
    #[serde(alias = "amt")]
    pub amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct RewardSlotHolder(String);
