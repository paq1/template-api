use crate::api::{{aggregate_name}}::{{aggregate_name}}_dbo::{{aggregate_name | capitalize}}DboState;
use crate::core::{{aggregate_name}}::repositories::Custom{{aggregate_name | capitalize}}Repository;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::MongoEntityRepository;

pub type Mongo{{aggregate_name | capitalize}}Repository = MongoEntityRepository<{{aggregate_name | capitalize}}DboState>;

#[async_trait]
impl Custom{{aggregate_name | capitalize}}Repository for Mongo{{aggregate_name | capitalize}}Repository {
}
