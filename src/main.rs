use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use framework_cqrs_lib::cqrs::infra::api_key::component::ApiKeyComponent;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::pouet::routes::read_routes::{fetch_many_pouet, fetch_one_pouet};
use api::pouet::routes::write_routes::insert_one_pouet;

use crate::api::pouet::pouet_component::RegexWordComponent;
use crate::api::pouet::routes::exemple_wit_api_key_routes::exemple_api_key;
use crate::api::pouet::routes::read_routes::{fetch_pouet_events, fetch_one_pouet_event};
use crate::api::swagger::ApiDoc;
use framework_cqrs_lib::cqrs::infra::authentication::AuthenticationComponent;


use framework_cqrs_lib::cqrs::models::errors::StandardHttpError;
use log::info;

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("lancement du server");

    let authentication_component = Arc::new(AuthenticationComponent::new().unwrap());
    let api_key_component = Arc::new(ApiKeyComponent::new(
        "gonecodeapi", "{{aggregate_name}}",
    ).await);

    // {{aggregate_name}} aggregat
    let regexword_component = RegexWordComponent::new(&authentication_component.clone()).await;

    let openapi = ApiDoc::openapi();
    let api_address = std::env::var("API_ADDRESS").unwrap();
    let api_port = std::env::var("API_PORT").unwrap().parse::<u16>().unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .supports_credentials();

        let standard_http_error = StandardHttpError::new();

        App::new()
            .wrap(cors)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone(),
            ))
            .app_data(web::Data::new(standard_http_error))
            .app_data(web::Data::new(authentication_component.jwt_token_decoder_service.clone()))
            // {{aggregate_name}} services
            .service(
                web::scope("/{{aggregate_name}}")
                    .service(fetch_one_pouet)
                    .service(fetch_one_pouet_event)
                    .service(fetch_many_pouet)
                    .service(fetch_pouet_events)
                    .service(insert_one_pouet)
                    .service(exemple_api_key)
                    .app_data(web::Data::new(Arc::clone(&regexword_component.engine)))
                    .app_data(
                        web::Data::new(Arc::clone(&regexword_component.store))
                    )
                    .app_data(
                        web::Data::new(Arc::clone(&regexword_component.journal))
                    )
                    .app_data(
                        web::Data::new(Arc::clone(&regexword_component.service))
                    )
                    .app_data(
                        web::Data::new(api_key_component.service.clone())
                    )
            )
    })
        .workers(2)
        .bind((api_address.clone(), api_port.clone()))?
        .run()
        .await
}