pub mod rules;

use async_trait::async_trait;

#[async_trait]
pub trait {{aggregate_name | capitalize}}Service: Send + Sync {

}
