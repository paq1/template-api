use std::sync::Arc;

use crate::core::pouet::data::events::PouetEvents;
use crate::core::pouet::data::states::PouetStates;
use crate::models::pouet::commands::{PouetCommands, CreatePouetCommand};
use crate::models::pouet::views::PouetViewEvent;
use actix_web::{post, web, Responder};
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::engine::Engine;
use framework_cqrs_lib::cqrs::infra::helpers::http_response::{CanToHttpResponse, HttpKindResponse};
use framework_cqrs_lib::cqrs::infra::mappers::event_api_view::from_entity_event_to_view;
use uuid::Uuid;
use crate::core::pouet::command_handler::create_handler::PouetCreateHandler;

#[utoipa::path(
    tag = "{{aggregate_name}}",
    path = "/{{aggregate_name}}/commands/create",
    request_body = CreateRegexWordCommand,
    responses(
    (status = 201, description = "mettre la description ici", body = String),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[post("/commands/create")]
pub async fn insert_one_pouet(
    body: web::Json<CreatePouetCommand>,
    engine: web::Data<Arc<Engine<PouetStates, PouetCommands, PouetEvents>>>,
) -> impl Responder {
    let ctx = Context::empty();

    let command = PouetCommands::Create(body.into_inner());

    let entity_id = Uuid::new_v4().to_string();

    let event = engine
        .compute(command, entity_id, PouetCreateHandler::handler_name().to_string(), &ctx).await;

    event.map(|(event, _)| {
        from_entity_event_to_view::<PouetEvents, PouetViewEvent>(
            event,
            "{{aggregate_name}}".to_string(),
            "urn:api:{{aggregate_name}}:{{aggregate_name}}".to_string(),
            &ctx,
        )
    })
        .to_http_response_with_error_mapping(HttpKindResponse::Created)
}

