use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum {{aggregate_name | capitalize}}ViewState {
    #[serde(rename = "{{aggregate_name}}")]
    Create({{aggregate_name | capitalize}}View),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct {{aggregate_name | capitalize}}View {
    #[serde(flatten)]
    pub data: {{aggregate_name | capitalize}}DataView,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct {{aggregate_name | capitalize}}DataView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(tag = "eventType")]
pub enum {{aggregate_name | capitalize}}ViewEvent {
    #[serde(rename = "created")]
    Created({{aggregate_name | capitalize}}CreatedView),
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct {{aggregate_name | capitalize}}CreatedView {
    #[serde(flatten)]
    pub data: {{aggregate_name | capitalize}}DataView,
    pub by: String,
    pub at: DateTime<Utc>,
}
