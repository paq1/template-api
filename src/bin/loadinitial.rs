use std::sync::Arc;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;
use reqwest::Client;

#[tokio::main]
async fn main() -> ResultErr<()>{
    dotenv::dotenv().ok();
    env_logger::init();
    let _http_client: Arc<Client> = Arc::new(Client::new());
    Ok(())
}
