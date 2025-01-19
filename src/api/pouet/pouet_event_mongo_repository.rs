use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::api::pouet::pouet_dbo::PouetDboEvent;
use crate::api::pouet::mappers::events::{from_pouet_event_dbo_to_event, from_pouet_event_to_dbo};
use crate::core::pouet::data::events::PouetEvents;
use framework_cqrs_lib::cqrs::core::daos::DAO;
use framework_cqrs_lib::cqrs::core::data::EntityEvent;
use framework_cqrs_lib::cqrs::core::repositories::can_fetch_all::CanFetchAll;
use framework_cqrs_lib::cqrs::core::repositories::events::{ReadOnlyEventRepo, RepositoryEvents, WriteOnlyEventRepo};
use framework_cqrs_lib::cqrs::core::repositories::query::Query;
use framework_cqrs_lib::cqrs::core::repositories::CanFetchMany;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EventDBO;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;

pub struct PouetEventMongoRepository {
    pub dao: Arc<Mutex<dyn DAO<EventDBO<PouetDboEvent, String>, String>>>,
}

#[async_trait]
impl RepositoryEvents<PouetEvents, String> for PouetEventMongoRepository {}

#[async_trait]
impl CanFetchAll<EntityEvent<PouetEvents, String>> for PouetEventMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<EntityEvent<PouetEvents, String>>> {
        self.dao
            .lock().await
            .fetch_all(query)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(|dbo| from_pouet_event_dbo_to_event(dbo))
                    .collect()
            })
    }
}

#[async_trait]
impl CanFetchMany<EntityEvent<PouetEvents, String>> for PouetEventMongoRepository {}

#[async_trait]
impl ReadOnlyEventRepo<PouetEvents, String> for PouetEventMongoRepository {
    async fn fetch_one(&self, event_id: &String) -> ResultErr<Option<EntityEvent<PouetEvents, String>>> {
        self.dao.lock().await.fetch_one(event_id).await.map(|maybevent| {
            maybevent.map(|event_dbo| {
                from_pouet_event_dbo_to_event(event_dbo)
            })
        })
    }
}

#[async_trait]
impl WriteOnlyEventRepo<PouetEvents, String> for PouetEventMongoRepository {
    async fn insert(&self, entity_event: &EntityEvent<PouetEvents, String>) -> ResultErr<String> {
        let dao: EventDBO<PouetDboEvent, String> = from_pouet_event_to_dbo(entity_event.clone());

        let dao_sanitize_version: EventDBO<PouetDboEvent, String> = EventDBO {
            version: Some(0),
            ..dao.clone()
        };

        self.dao.lock().await.insert(&dao_sanitize_version, &dao.entity_id).await
    }
}