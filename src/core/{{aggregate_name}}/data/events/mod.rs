use chrono::{DateTime, Utc};

use crate::core::{{aggregate_name}}::data::{{aggregate_name}}_data::{{aggregate_name | capitalize}}Data;
use crate::models::{{aggregate_name}}::views::{{{aggregate_name | capitalize}}CreatedView, {{aggregate_name | capitalize}}DataView, {{aggregate_name | capitalize}}ViewEvent};
use framework_cqrs_lib::cqrs::models::jsonapi::CanBeView;

impl CanBeView<{{aggregate_name | capitalize}}ViewEvent> for {{aggregate_name | capitalize}}Events {
    fn to_view(&self) -> {{aggregate_name | capitalize}}ViewEvent {
        match self.clone() {
            {{aggregate_name | capitalize}}Events::Created(c) => {{aggregate_name | capitalize}}ViewEvent::Created({{aggregate_name | capitalize}}CreatedView {
                data: {{aggregate_name | capitalize}}DataView {
                    message: c.data.message,
                },
                by: c.by,
                at: c.at
            }),
        }
    }
}


#[derive(Clone)]
pub enum {{aggregate_name | capitalize}}Events {
    Created({{aggregate_name | capitalize}}Created),
}

#[derive(Clone)]
pub struct {{aggregate_name | capitalize}}Created {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: {{aggregate_name | capitalize}}Data,
}
