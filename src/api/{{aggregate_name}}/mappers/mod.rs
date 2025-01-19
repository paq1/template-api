use crate::api::{{aggregate_name}}::{{aggregate_name}}_dbo::{{aggregate_name | capitalize}}DataDbo;
use crate::core::{{aggregate_name}}::data::{{aggregate_name}}_data::{{aggregate_name | capitalize}}Data;

pub mod states;
pub mod events;

impl From<{{aggregate_name | capitalize}}DataDbo> for {{aggregate_name | capitalize}}Data {
    fn from(value: {{aggregate_name | capitalize}}DataDbo) -> Self {
        Self {
            message: value.message,
        }
    }
}

impl From<{{aggregate_name | capitalize}}Data> for {{aggregate_name | capitalize}}DataDbo {
    fn from(value: {{aggregate_name | capitalize}}Data) -> Self {
        Self {
            message: value.message,
        }
    }
}

