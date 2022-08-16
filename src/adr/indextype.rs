use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ValueEnum, Debug, Clone)]
pub enum IndexType {
    Timestamp,
    Sequential,
}
