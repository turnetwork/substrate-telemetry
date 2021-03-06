use chrono::{DateTime, Utc};
use serde::Deserialize;

pub use primitive_types::H256 as BlockHash;
pub type BlockNumber = u64;

#[derive(Deserialize, Debug)]
pub struct NodeMessage {
    level: Level,
    ts: DateTime<Utc>,
    #[serde(flatten)]
    details: Details,
}

#[derive(Deserialize, Debug)]
pub enum Level {
    #[serde(rename = "INFO")]
    Info,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "msg")]
pub enum Details {
    #[serde(rename = "node.start")]
    NodeStart(BestBlock),
    #[serde(rename = "system.connected")]
    SystemConnected(SystemConnected),
    #[serde(rename = "system.interval")]
    SystemInterval(SystemInterval),
    #[serde(rename = "block.import")]
    BlockImport(BestBlock),
}

#[derive(Deserialize, Debug)]
pub struct SystemConnected {
    pub name: Box<str>,
    pub chain: Box<str>,
    pub implementation: Box<str>,
    pub version: Box<str>,
    pub config: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
pub struct SystemInterval {
    pub txcount: u64,
    pub peers: u64,
    pub memory: Option<f64>,
    pub cpu: Option<f64>,
    pub bandwidth_upload: Option<f64>,
    pub bandwidth_download: Option<f64>,
    pub finalized_height: Option<BlockNumber>,
    pub finalized_hash: Option<BlockHash>,
    #[serde(flatten)]
    pub best_block: BestBlock,
}

#[derive(Deserialize, Debug)]
pub struct BestBlock {
    pub best: BlockHash,
    pub height: BlockNumber,
}
