use std::collections::HashMap;
use std::iter::Iterator;
use std::sync::Arc;

use actix_web::web::Query;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::api::pouet::query::{from_pouet_query_to_query_repo, RegexWordQuery};
use crate::core::pouet::data::events::PouetEvents;
use crate::core::pouet::repositories::CustomPouetRepository;
use crate::models::pouet::views::{PouetViewEvent, PouetViewState};
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::repositories::events::RepositoryEvents;
use framework_cqrs_lib::cqrs::core::repositories::filter::{Expr, ExprGeneric, Filter, Operation};
use framework_cqrs_lib::cqrs::core::repositories::query::{Paged, Query as QueryCore};
use framework_cqrs_lib::cqrs::infra::helpers::context::CanDecoreFromHttpRequest;
use framework_cqrs_lib::cqrs::infra::mappers::event_api_view::from_entity_event_to_view;
use framework_cqrs_lib::cqrs::infra::mappers::state_view::{
    from_states_to_entity_view, from_states_to_view, CanBeManyView,
};
use framework_cqrs_lib::cqrs::models::errors::StandardHttpError;
use framework_cqrs_lib::cqrs::models::jsonapi::CanBeView;
use framework_cqrs_lib::cqrs::models::views::entities::EntityView;

#[utoipa::path(
    tag = "{{aggregate_name}}",
    path = "/{{aggregate_name}}",
    responses(
        (status = 200, description = "fait ca", body = Paged<EntityView<RegexWordViewState>>)
    ),
    params(
        RegexWordQuery
    )
)]
#[get("")]
pub async fn fetch_many_pouet(
    store: web::Data<Arc<dyn CustomPouetRepository>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<RegexWordQuery>,
    req: HttpRequest,
) -> impl Responder {
    let ctx: Context = Context::empty()
        .decore_with_http_header(&req)
        .clone_with_filter(HashMap::from([
            (
                "page[number]".to_string(),
                query
                    .number
                    .map(|x| x.to_string())
                    .unwrap_or("0".to_string()),
            ),
            (
                "page[size]".to_string(),
                query
                    .size
                    .map(|x| x.to_string())
                    .unwrap_or("10".to_string()),
            ),
        ]));

    match store
        .fetch_many(from_pouet_query_to_query_repo(query))
        .await
    {
        Ok(items) => {
            let paged_view: Paged<EntityView<PouetViewState>> =
                items.map(|entity| from_states_to_entity_view(entity, "{{aggregate_name}}".to_string(), &ctx));

            HttpResponse::Ok().json(paged_view.to_many_view(
                &ctx,
                "{{aggregate_name}}".to_string(),
                HashMap::from([("{{aggregate_name}}".to_string(), "{{aggregate_name}}".to_string())]),
            ))
        }
        Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error),
    }
}

#[utoipa::path(
    tag = "{{aggregate_name}}",
    path = "/{{aggregate_name}}/{entity_id}",
    responses(
        (
        status = 200,
        description = "Get the current state.",
        body = RegexWordStates
        )
    )
)]
#[get("/{entity_id}")]
pub async fn fetch_one_pouet(
    path: web::Path<String>,
    store: web::Data<Arc<dyn CustomPouetRepository>>,
    http_error: web::Data<StandardHttpError>,
    req: HttpRequest,
) -> impl Responder {
    let id = path.into_inner();

    let ctx = Context::empty().decore_with_http_header(&req);

    match store.fetch_one(&id).await {
        Ok(Some(entity)) => {
            let view = from_states_to_view(entity, "{{aggregate_name}}".to_string(), &ctx);

            HttpResponse::Ok().json(view)
        }
        Ok(_) => HttpResponse::NotFound().json(&http_error.not_found),
        Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error),
    }
}

#[utoipa::path(
    tag = "{{aggregate_name}}",
    path = "/{{aggregate_name}}/{entity_id}/events",
    responses(
        (
        status = 200,
        description = "Get the all events ",
        body = RegexWordView
        )
    ),
    params(
        RegexWordQuery
    )
)]
#[get("/{entity_id}/events")]
pub async fn fetch_pouet_events(
    path: web::Path<String>,
    journal: web::Data<Arc<dyn RepositoryEvents<PouetEvents, String>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<RegexWordQuery>,
    req: HttpRequest,
) -> impl Responder {
    let id = path.into_inner();
    let query_core: QueryCore = from_pouet_query_to_query_repo(query.clone());

    let ctx: Context = Context::empty()
        .decore_with_http_header(&req)
        .clone_with_filter(HashMap::from([
            (
                "page[number]".to_string(),
                query
                    .number
                    .map(|x| x.to_string())
                    .unwrap_or("0".to_string()),
            ),
            (
                "page[size]".to_string(),
                query
                    .size
                    .map(|x| x.to_string())
                    .unwrap_or("10".to_string()),
            ),
        ]));

    let query_core_with_filter = QueryCore {
        filter: Filter::Expr(Expr::ExprStr(ExprGeneric::<String> {
            field: "entity_id".to_string(),
            operation: Operation::EqualsTo,
            head: id.to_string(),
        })),
        ..query_core
    };

    match journal.fetch_many(query_core_with_filter).await {
        Ok(items) => {
            let paged_view = items.map(|entity_event| EntityView {
                r#type: "urn:api:{{aggregate_name}}:{{aggregate_name}}".to_string(),
                id: entity_event.entity_id,
                attributes: entity_event.data.to_view(),
                links: None,
            });

            HttpResponse::Ok().json(paged_view.to_many_view(
                &ctx,
                "{{aggregate_name}}".to_string(),
                HashMap::new(),
            ))
        }
        Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error),
    }
}

#[utoipa::path(
    tag = "{{aggregate_name}}",
    path = "/{{aggregate_name}}/{entity_id}/events/{event_id}",
    responses(
        (
        status = 200,
        description = "Get event.",
        body = DataWrapperView < EventView < RegexWordViewEvent >>
        )
    )
)]
#[get("/{entity_id}/events/{event_id}")]
pub async fn fetch_one_pouet_event(
    path: web::Path<(String, String)>,
    journal: web::Data<Arc<dyn RepositoryEvents<PouetEvents, String>>>,
    http_error: web::Data<StandardHttpError>,
    req: HttpRequest,
) -> impl Responder {
    let (_, event_id) = path.into_inner();

    let ctx = Context::empty().decore_with_http_header(&req);

    match journal.fetch_one(&event_id).await {
        Ok(maybe_event) => match maybe_event {
            Some(event) => {
                let view = from_entity_event_to_view::<PouetEvents, PouetViewEvent>(
                    event,
                    "{{aggregate_name}}".to_string(),
                    "urn:api:{{aggregate_name}}:{{aggregate_name}}".to_string(),
                    &ctx,
                );
                HttpResponse::Ok().json(view)
            }
            None => HttpResponse::InternalServerError().json(&http_error.not_found),
        },
        Err(_) => HttpResponse::InternalServerError().json(&http_error.internal_server_error),
    }
}
