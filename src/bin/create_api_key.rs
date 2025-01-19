use framework_cqrs_lib::cqrs::infra::api_key::component::ApiKeyComponent;
use std::sync::Arc;
use log::{error, info};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let api_key_component = Arc::new(ApiKeyComponent::new(
        "gonecodeapi", "{{aggregate_name}}",
    ).await);

    let maybe_create_api_key = api_key_component
        .service
        .create_api_key(&"whowhantapi-accesss-exemple-doggoverselol".to_string()).await;

    match maybe_create_api_key {
        Ok(api_key) => {
            info!("Successfully created api key");
            info!("{:?}", api_key);
        }
        Err(err) => {
            error!("Error creating api key: {:?}", err);
        }
    }
}
