use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct {{aggregate_name | capitalize}}DataDbo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum {{aggregate_name | capitalize}}DboState {
    {{aggregate_name | capitalize}}Dbo {
        #[serde(rename = "_kind")]
        kind: String,
        #[serde(flatten)]
        data: {{aggregate_name | capitalize}}DataDbo,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum {{aggregate_name | capitalize}}DboEvent {
    Created({{aggregate_name | capitalize}}CreatedDbo),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct {{aggregate_name | capitalize}}CreatedDbo {
    pub by: String,
    pub at: DateTime<Utc>,
    #[serde(flatten)]
    pub data: {{aggregate_name | capitalize}}DataDbo,
}
