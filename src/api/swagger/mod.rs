use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

use crate::api::{{aggregate_name}}::routes::exemple_wit_api_key_routes::__path_exemple_api_key;
use crate::api::{{aggregate_name}}::routes::read_routes::__path_fetch_{{aggregate_name}}_events;
use crate::api::{{aggregate_name}}::routes::read_routes::__path_fetch_many_{{aggregate_name}};
use crate::api::{{aggregate_name}}::routes::read_routes::__path_fetch_one_{{aggregate_name}};
use crate::api::{{aggregate_name}}::routes::write_routes::__path_insert_one_{{aggregate_name}};
use crate::models::{{aggregate_name}}::commands::*;
use crate::models::{{aggregate_name}}::views::*;
use framework_cqrs_lib::cqrs::core::repositories::query::{InfoPaged, Page};
use framework_cqrs_lib::cqrs::models::api_key::commands::CreateApiKeyCommand;
use framework_cqrs_lib::cqrs::models::jsonapi::ManyView;
use framework_cqrs_lib::cqrs::models::views::command_handler_view::ApiView;
use framework_cqrs_lib::cqrs::models::views::DataWrapperView;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        fetch_many_{{aggregate_name}},
        fetch_one_{{aggregate_name}},
        insert_one_{{aggregate_name}},
        fetch_{{aggregate_name}}_events,
        exemple_api_key,
    ),
    components(
        schemas(
            CreateApiKeyCommand,
            Create{{aggregate_name | capitalize}}Command,
            {{aggregate_name | capitalize}}CreatedView,
            ManyView < {{aggregate_name | capitalize}}ViewState >,
            Create{{aggregate_name | capitalize}}Command,
            DataWrapperView < ApiView < {{aggregate_name | capitalize}}ViewEvent > >,
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