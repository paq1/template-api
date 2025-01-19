use crate::api::pouet::pouet_dbo::PouetDboState;
use crate::core::pouet::repositories::CustomPouetRepository;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::MongoEntityRepository;

pub type MongoPouetRepository = MongoEntityRepository<PouetDboState>;

#[async_trait]
impl CustomPouetRepository for MongoPouetRepository {
}
