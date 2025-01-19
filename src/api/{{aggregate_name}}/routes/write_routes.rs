use std::sync::Arc;

use crate::core::{{aggregate_name}}::data::events::{{aggregate_name | capitalize}}Events;
use crate::core::{{aggregate_name}}::data::states::{{aggregate_name | capitalize}}States;
use crate::models::{{aggregate_name}}::commands::{
  {{aggregate_name | capitalize}}Commands,
  Create{{aggregate_name | capitalize}}Command
};
use crate::models::{{aggregate_name}}::views::{{aggregate_name | capitalize}}ViewEvent;
use actix_web::{post, web, Responder};
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::event_sourcing::engine::Engine;
use framework_cqrs_lib::cqrs::infra::helpers::http_response::{CanToHttpResponse, HttpKindResponse};
use framework_cqrs_lib::cqrs::infra::mappers::event_api_view::from_entity_event_to_view;
use uuid::Uuid;
use crate::core::{{aggregate_name}}::command_handler::create_handler::{{aggregate_name | capitalize}}CreateHandler;

#[utoipa::path(
    tag = "{{aggregate_name}}",
    path = "/{{aggregate_name}}/commands/create",
    request_body = Create{{aggregate_name | capitalize}}Command,
    responses(
    (status = 201, description = "mettre la description ici", body = String),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[post("/commands/create")]
pub async fn insert_one_{{aggregate_name}}(
    body: web::Json<Create{{aggregate_name | capitalize}}Command>,
    engine: web::Data<Arc<Engine<{{aggregate_name | capitalize}}States, {{aggregate_name | capitalize}}Commands, {{aggregate_name | capitalize}}Events>>>,
) -> impl Responder {
    let ctx = Context::empty();

    let command = {{aggregate_name | capitalize}}Commands::Create(body.into_inner());

    let entity_id = Uuid::new_v4().to_string();

    let event = engine
        .compute(command, entity_id, {{aggregate_name | capitalize}}CreateHandler::handler_name().to_string(), &ctx).await;

    event.map(|(event, _)| {
        from_entity_event_to_view::<{{aggregate_name | capitalize}}Events, {{aggregate_name | capitalize}}ViewEvent>(
            event,
            "{{aggregate_name}}".to_string(),
            "urn:api:{{aggregate_name}}:{{aggregate_name}}".to_string(),
            &ctx,
        )
    })
        .to_http_response_with_error_mapping(HttpKindResponse::Created)
}

