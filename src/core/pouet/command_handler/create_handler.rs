use async_trait::async_trait;
use std::sync::Arc;

use crate::core::pouet::data::events::{PouetCreated, PouetEvents};
use crate::core::pouet::data::pouet_data::PouetData;
use crate::core::pouet::data::states::PouetStates;
use crate::core::pouet::services::rules::PouetRules;
use crate::models::pouet::commands::PouetCommands;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandlerCreate;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;

pub struct PouetCreateHandler {
    pub rules: Arc<dyn PouetRules>,
}
#[async_trait]
impl CommandHandlerCreate<PouetStates, PouetCommands, PouetEvents> for PouetCreateHandler {
    fn name(&self) -> String {
        Self::handler_name().to_string()
    }

    async fn on_command(
        &self,
        _id: String,
        command: PouetCommands,
        context: &Context,
    ) -> ResultErr<PouetEvents> {
        match command {
            PouetCommands::Create(c) => Ok(PouetEvents::Created(PouetCreated {
                by: context.clone().subject,
                at: context.clone().now,
                data: PouetData { message: c.message },
            })),
        }
    }
}

impl PouetCreateHandler {
    pub fn handler_name() -> &'static str {
        "create-{{aggregate_name}}"
    }
}
