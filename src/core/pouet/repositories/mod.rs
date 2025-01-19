use crate::core::pouet::data::states::PouetStates;
use async_trait::async_trait;
use framework_cqrs_lib::cqrs::core::repositories::entities::RepositoryEntity;

#[async_trait]
pub trait CustomPouetRepository: RepositoryEntity<PouetStates, String> {
}
