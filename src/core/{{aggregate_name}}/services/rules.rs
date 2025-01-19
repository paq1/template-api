use async_trait::async_trait;

#[async_trait]
pub trait {{aggregate_name | capitalize}}Rules: Sync + Send {

}