use async_trait::async_trait;
use std::sync::Arc;

use crate::core::{{aggregate_name}}::data::events::{
  {{aggregate_name | capitalize}}Created,
  {{aggregate_name | capitalize}}Events
};
use crate::core::{{aggregate_name}}::data::{{aggregate_name}}_data::{{aggregate_name | capitalize}}Data;
use crate::core::{{aggregate_name}}::data::states::{{aggregate_name | capitalize}}States;
use crate::core::{{aggregate_name}}::services::rules::{{aggregate_name | capitalize}}Rules;
use crate::models::{{aggregate_name}}::commands::{{aggregate_name | capitalize}}Commands;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerCreate;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;

pub struct {{aggregate_name | capitalize}}CreateHandler {
    pub rules: Arc<dyn {{aggregate_name | capitalize}}Rules>,
}
#[async_trait]
impl CommandHandlerCreate<{{aggregate_name | capitalize}}States, {{aggregate_name | capitalize}}Commands, {{aggregate_name | capitalize}}Events> for {{aggregate_name | capitalize}}CreateHandler {
    fn name(&self) -> String {
        Self::handler_name().to_string()
    }

    async fn on_command(
        &self,
        _id: String,
        command: {{aggregate_name | capitalize}}Commands,
        context: &Context,
    ) -> ResultErr<{{aggregate_name | capitalize}}Events> {
        match command {
            {{aggregate_name | capitalize}}Commands::Create(c) => Ok({{aggregate_name | capitalize}}Events::Created({{aggregate_name | capitalize}}Created {
                by: context.clone().subject,
                at: context.clone().now,
                data: {{aggregate_name | capitalize}}Data { message: c.message },
            })),
        }
    }
}

impl {{aggregate_name | capitalize}}CreateHandler {
    pub fn handler_name() -> &'static str {
        "create-{{aggregate_name}}"
    }
}
