use clap::ValueEnum;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, ValueEnum, Debug, Clone)]
pub enum IndexType {
    Timestamp,
    Sequential,
}
