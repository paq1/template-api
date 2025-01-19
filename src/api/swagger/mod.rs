use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

use crate::api::pouet::routes::exemple_wit_api_key_routes::__path_exemple_api_key;
use crate::api::pouet::routes::read_routes::__path_fetch_pouet_events;
use crate::api::pouet::routes::read_routes::__path_fetch_many_pouet;
use crate::api::pouet::routes::read_routes::__path_fetch_one_pouet;
use crate::api::pouet::routes::write_routes::__path_insert_one_pouet;
use crate::models::pouet::commands::*;
use crate::models::pouet::views::*;
use framework_cqrs_lib::cqrs::core::repositories::query::{InfoPaged, Page};
use framework_cqrs_lib::cqrs::models::api_key::commands::CreateApiKeyCommand;
use framework_cqrs_lib::cqrs::models::jsonapi::ManyView;
use framework_cqrs_lib::cqrs::models::views::command_handler_view::ApiView;
use framework_cqrs_lib::cqrs::models::views::DataWrapperView;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        fetch_many_pouet,
        fetch_one_pouet,
        insert_one_pouet,
        fetch_pouet_events,
        exemple_api_key,
    ),
    components(
        schemas(
            CreateApiKeyCommand,
            CreatePouetCommand,
            PouetCreatedView,
            ManyView < PouetViewState >,
            CreatePouetCommand,
            DataWrapperView < ApiView < PouetViewEvent > >,
            InfoPaged,
            Page
        )
    ),
    modifiers(& SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {

        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build()
            ),
        );
        components.add_security_scheme(
            "api_key_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-KEY"))),
        )
    }
}