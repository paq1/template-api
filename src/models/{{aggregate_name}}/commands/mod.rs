use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum {{aggregate_name | capitalize}}Commands {
    Create(Create{{aggregate_name | capitalize}}Command),
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Create{{aggregate_name | capitalize}}Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
