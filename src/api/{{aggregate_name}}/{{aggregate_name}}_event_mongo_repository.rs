use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::api::{{aggregate_name}}::{{aggregate_name}}_dbo::{{aggregate_name | capitalize}}DboEvent;
use crate::api::{{aggregate_name}}::mappers::events::{from_{{aggregate_name}}_event_dbo_to_event, from_{{aggregate_name}}_event_to_dbo};
use crate::core::{{aggregate_name}}::data::events::{{aggregate_name | capitalize}}Events;
use framework_cqrs_lib::cqrs::core::daos::DAO;
use framework_cqrs_lib::cqrs::core::data::EntityEvent;
use framework_cqrs_lib::cqrs::core::repositories::can_fetch_all::CanFetchAll;
use framework_cqrs_lib::cqrs::core::repositories::events::{ReadOnlyEventRepo, RepositoryEvents, WriteOnlyEventRepo};
use framework_cqrs_lib::cqrs::core::repositories::query::Query;
use framework_cqrs_lib::cqrs::core::repositories::CanFetchMany;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EventDBO;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;

pub struct {{aggregate_name | capitalize}}EventMongoRepository {
    pub dao: Arc<Mutex<dyn DAO<EventDBO<{{aggregate_name | capitalize}}DboEvent, String>, String>>>,
}

#[async_trait]
impl RepositoryEvents<{{aggregate_name | capitalize}}Events, String> for {{aggregate_name | capitalize}}EventMongoRepository {}

#[async_trait]
impl CanFetchAll<EntityEvent<{{aggregate_name | capitalize}}Events, String>> for {{aggregate_name | capitalize}}EventMongoRepository {
    async fn fetch_all(&self, query: Query) -> ResultErr<Vec<EntityEvent<{{aggregate_name | capitalize}}Events, String>>> {
        self.dao
            .lock().await
            .fetch_all(query)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(|dbo| from_{{aggregate_name}}_event_dbo_to_event(dbo))
                    .collect()
            })
    }
}

#[async_trait]
impl CanFetchMany<EntityEvent<{{aggregate_name | capitalize}}Events, String>> for {{aggregate_name | capitalize}}EventMongoRepository {}

#[async_trait]
impl ReadOnlyEventRepo<{{aggregate_name | capitalize}}Events, String> for {{aggregate_name | capitalize}}EventMongoRepository {
    async fn fetch_one(&self, event_id: &String) -> ResultErr<Option<EntityEvent<{{aggregate_name | capitalize}}Events, String>>> {
        self.dao.lock().await.fetch_one(event_id).await.map(|maybevent| {
            maybevent.map(|event_dbo| {
                from_{{aggregate_name}}_event_dbo_to_event(event_dbo)
            })
        })
    }
}

#[async_trait]
impl WriteOnlyEventRepo<{{aggregate_name | capitalize}}Events, String> for {{aggregate_name | capitalize}}EventMongoRepository {
    async fn insert(&self, entity_event: &EntityEvent<{{aggregate_name | capitalize}}Events, String>) -> ResultErr<String> {
        let dao: EventDBO<{{aggregate_name | capitalize}}DboEvent, String> = from_{{aggregate_name}}_event_to_dbo(entity_event.clone());

        let dao_sanitize_version: EventDBO<{{aggregate_name | capitalize}}DboEvent, String> = EventDBO {
            version: Some(0),
            ..dao.clone()
        };

        self.dao.lock().await.insert(&dao_sanitize_version, &dao.entity_id).await
    }
}