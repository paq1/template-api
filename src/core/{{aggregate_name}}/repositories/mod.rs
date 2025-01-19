use crate::core::{{aggregate_name}}::data::states::{{aggregate_name | capitalize}}States;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::repositories::entities::RepositoryEntity;

#[async_trait]
pub trait Custom{{aggregate_name | capitalize}}Repository: RepositoryEntity<{{aggregate_name | capitalize}}States, String> {
}
