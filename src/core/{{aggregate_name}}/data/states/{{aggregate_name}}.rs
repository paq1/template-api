use crate::core::{{aggregate_name}}::data::events::{{aggregate_name | capitalize}}Events;
use crate::core::{{aggregate_name}}::data::{{aggregate_name}}_data::{{aggregate_name | capitalize}}Data;
use crate::core::{{aggregate_name}}::data::states::{{aggregate_name | capitalize}}States;
use crate::models::{{aggregate_name}}::views::{
  {{aggregate_name | capitalize}}DataView,
  {{aggregate_name | capitalize}}View
};
use framework_cqrs_lib::cqrs::models::jsonapi::CanGetTypee;

#[derive(Clone, Debug)]
pub struct {{aggregate_name | capitalize}} {
    pub kind: String,
    pub data: {{aggregate_name | capitalize}}Data,
}

impl {{aggregate_name | capitalize}} {
    pub fn reduce_state(&self, event: {{aggregate_name | capitalize}}Events) -> Option<{{aggregate_name | capitalize}}States> {
        // on accept pas d'evenement apres la creation
        match event {
            _ => None // illegal transition
        }
    }
}

impl CanGetTypee for {{aggregate_name | capitalize}} {
    fn get_type(&self) -> String {
        "urn:api:{{aggregate_name}}:{{aggregate_name}}".to_string()
    }
}

impl From<{{aggregate_name | capitalize}}> for {{aggregate_name | capitalize}}View {
    fn from(value: {{aggregate_name | capitalize}}) -> Self {
        {{aggregate_name | capitalize}}View {
            data: value.data.into(),
        }
    }
}

impl From<{{aggregate_name | capitalize}}Data> for {{aggregate_name | capitalize}}DataView {
    fn from(value: {{aggregate_name | capitalize}}Data) -> Self {
        Self {
            message: value.message,
        }
    }
}

