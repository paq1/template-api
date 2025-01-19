use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use framework_cqrs_lib::cqrs::core::api_key::services::api_key_service::ApiKeyService;
use framework_cqrs_lib::cqrs::infra::helpers::header_value::CanSanitizeHeader;
use std::sync::Arc;

#[utoipa::path(
    tag = "{{aggregate_name}} exemple api key",
    path = "/{{aggregate_name}}/exemple-api-key",
    responses(
    (status = 201, description = "mettre la description ici", body = String),
    ),
    security(
    ("api_key_auth" = [])
    )
)]
#[post("/exemple-api-key")]
pub async fn exemple_api_key(
    req: HttpRequest,
    api_key_service: web::Data<Arc<dyn ApiKeyService>>,
) -> impl Responder {
    let maybe_api_key: Option<String> = req
        .headers()
        .get("X-API-KEY")
        .map(|header_value| {
            header_value.clone()
                .sanitize_header("X-API-KEY".to_string())
                .map(|x| x.1)
                .unwrap_or("invalid_key".to_string())
        });

    match maybe_api_key {
        Some(api_key) => {
            match api_key_service.is_authorized(&api_key).await {
                Ok(true) => {
                    HttpResponse::Ok().body(format!("félicitation vous etes authoriser ici"))
                }
                Ok(false) => {
                    HttpResponse::Ok().body(format!("vous n'etes pas authorisé ici"))
                }
                _ => {
                    HttpResponse::InternalServerError().body(format!("erreur lors de l'authentification"))
                }
            }
        }
        None => {
            HttpResponse::Ok().body(format!("authentification sans api key"))
        }
    }
}
