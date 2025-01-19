use crate::api::{{aggregate_name}}::{{aggregate_name}}_dbo::{
  {{aggregate_name | capitalize}}DboEvent,
  {{aggregate_name | capitalize}}DboState
};
use crate::api::{{aggregate_name}}::{{aggregate_name}}_event_mongo_repository::{{aggregate_name | capitalize}}EventMongoRepository;
use crate::api::{{aggregate_name}}::{{aggregate_name}}_mongo_dao::{
  {{aggregate_name | capitalize}}EventMongoDAO,
  {{aggregate_name | capitalize}}MongoDAO
};
use crate::api::{{aggregate_name}}::{{aggregate_name}}_mongo_repository::Mongo{{aggregate_name | capitalize}}Repository;
use crate::api::{{aggregate_name}}::services::rules::RulesImpl;

use crate::api::{{aggregate_name}}::services::{{aggregate_name | capitalize}}ServiceImpl;
use crate::core::{{aggregate_name}}::command_handler::create_handler::{{aggregate_name | capitalize}}CreateHandler;

use crate::core::{{aggregate_name}}::data::events::{{aggregate_name | capitalize}}Events;
use crate::core::{{aggregate_name}}::data::states::{{aggregate_name | capitalize}}States;
use crate::core::{{aggregate_name}}::reducer::{{aggregate_name | capitalize}}Reducer;
use crate::core::{{aggregate_name}}::repositories::Custom{{aggregate_name | capitalize}}Repository;
use crate::core::{{aggregate_name}}::services::rules::{{aggregate_name | capitalize}}Rules;

use crate::core::{{aggregate_name}}::services::{{aggregate_name | capitalize}}Service;
use crate::models::{{aggregate_name}}::commands::{{aggregate_name | capitalize}}Commands;
use framework_cqrs_lib::cqrs::core::daos::DAO;
use framework_cqrs_lib::cqrs::core::event_sourcing::engine::Engine;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandler;
use framework_cqrs_lib::cqrs::core::repositories::events::RepositoryEvents;
use framework_cqrs_lib::cqrs::infra::authentication::AuthenticationComponent;
use framework_cqrs_lib::cqrs::infra::daos::database_mongo::DatabaseMongo;
use framework_cqrs_lib::cqrs::infra::daos::dbos::{EntityDBO, EventDBO};
use futures::lock::Mutex;
use std::sync::Arc;

pub struct {{aggregate_name | capitalize}}Component {
    pub store: Arc<dyn Custom{{aggregate_name | capitalize}}Repository>,
    pub journal: Arc<dyn RepositoryEvents<{{aggregate_name | capitalize}}Events, String>>,
    pub service: Arc<dyn {{aggregate_name | capitalize}}Service>,
    pub engine: Arc<Engine<{{aggregate_name | capitalize}}States, {{aggregate_name | capitalize}}Commands, {{aggregate_name | capitalize}}Events>>,
}

impl {{aggregate_name | capitalize}}Component {
    pub async fn new(_authentication_component: &Arc<AuthenticationComponent>) -> Self {
        let dbname = "gonecodeapi";

        let mongo_database = DatabaseMongo::new(dbname).await;

        let dao_store: Arc<dyn DAO<EntityDBO<{{aggregate_name | capitalize}}DboState, String>, String>> =
            Arc::new({{aggregate_name | capitalize}}MongoDAO::new(&mongo_database.underlying, "{{aggregate_name}}_store").await);
        let dao_journal: Arc<Mutex<dyn DAO<EventDBO<{{aggregate_name | capitalize}}DboEvent, String>, String>>> = Arc::new(
            Mutex::new({{aggregate_name | capitalize}}EventMongoDAO::new(&mongo_database.underlying, "{{aggregate_name}}_journal").await),
        );

        // repo
        let store = Arc::new(Mongo{{aggregate_name | capitalize}}Repository {
            dao: Arc::clone(&dao_store),
            database: mongo_database.underlying.clone(),
        });

        let journal: Arc<dyn RepositoryEvents<{{aggregate_name | capitalize}}Events, String>> =
            Arc::new({{aggregate_name | capitalize}}EventMongoRepository {
                dao: Arc::clone(&dao_journal),
            });

        // services
        let service: Arc<dyn {{aggregate_name | capitalize}}Service> = Arc::new({{aggregate_name | capitalize}}ServiceImpl {});

        let rules: Arc<dyn {{aggregate_name | capitalize}}Rules> = Arc::new(RulesImpl {
            store: store.clone(),
        });

        let engine: Arc<Engine<{{aggregate_name | capitalize}}States, {{aggregate_name | capitalize}}Commands, {{aggregate_name | capitalize}}Events>> = Arc::new(Engine {
            handlers: vec![CommandHandler::Create(Box::new({{aggregate_name | capitalize}}CreateHandler {
                rules: rules.clone(),
            }))],
            reducer: {{aggregate_name | capitalize}}Reducer::new().underlying,
            store: store.clone(),
            journal: journal.clone(),
        });

        Self {
            store,
            journal,
            service,
            engine,
        }
    }
}
