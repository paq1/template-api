use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PouetDataDbo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum PouetDboState {
    PouetDbo {
        #[serde(rename = "_kind")]
        kind: String,
        #[serde(flatten)]
        data: PouetDataDbo,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum PouetDboEvent {
    Created(PouetCreatedDbo),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PouetCreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: PouetDataDbo,
}
