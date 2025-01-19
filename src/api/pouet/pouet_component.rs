use crate::api::pouet::pouet_dbo::{PouetDboEvent, PouetDboState};
use crate::api::pouet::pouet_event_mongo_repository::PouetEventMongoRepository;
use crate::api::pouet::pouet_mongo_dao::{PouetEventMongoDAO, PouetMongoDAO};
use crate::api::pouet::pouet_mongo_repository::MongoPouetRepository;
use crate::api::pouet::services::rules::RulesImpl;

use crate::api::pouet::services::PouetServiceImpl;
use crate::core::pouet::command_handler::create_handler::PouetCreateHandler;

use crate::core::pouet::data::events::PouetEvents;
use crate::core::pouet::data::states::PouetStates;
use crate::core::pouet::reducer::RegexWordReducer;
use crate::core::pouet::repositories::CustomPouetRepository;
use crate::core::pouet::services::rules::PouetRules;

use crate::core::pouet::services::PouetService;
use crate::models::pouet::commands::PouetCommands;
use framework_cqrs_lib::cqrs::core::daos::DAO;
use framework_cqrs_lib::cqrs::core::event_sourcing::engine::Engine;
use framework_cqrs_lib::cqrs::core::event_sourcing::CommandHandler;
use framework_cqrs_lib::cqrs::core::repositories::events::RepositoryEvents;
use framework_cqrs_lib::cqrs::infra::authentication::AuthenticationComponent;
use framework_cqrs_lib::cqrs::infra::daos::database_mongo::DatabaseMongo;
use framework_cqrs_lib::cqrs::infra::daos::dbos::{EntityDBO, EventDBO};
use futures::lock::Mutex;
use std::sync::Arc;

pub struct RegexWordComponent {
    pub store: Arc<dyn CustomPouetRepository>,
    pub journal: Arc<dyn RepositoryEvents<PouetEvents, String>>,
    pub service: Arc<dyn PouetService>,
    pub engine: Arc<Engine<PouetStates, PouetCommands, PouetEvents>>,
}

impl RegexWordComponent {
    pub async fn new(_authentication_component: &Arc<AuthenticationComponent>) -> Self {
        let dbname = "gonecodeapi";

        let mongo_database = DatabaseMongo::new(dbname).await;

        let dao_store: Arc<dyn DAO<EntityDBO<PouetDboState, String>, String>> =
            Arc::new(PouetMongoDAO::new(&mongo_database.underlying, "pouet_store").await);
        let dao_journal: Arc<Mutex<dyn DAO<EventDBO<PouetDboEvent, String>, String>>> = Arc::new(
            Mutex::new(PouetEventMongoDAO::new(&mongo_database.underlying, "pouet_journal").await),
        );

        // repo
        let store = Arc::new(MongoPouetRepository {
            dao: Arc::clone(&dao_store),
            database: mongo_database.underlying.clone(),
        });

        let journal: Arc<dyn RepositoryEvents<PouetEvents, String>> =
            Arc::new(PouetEventMongoRepository {
                dao: Arc::clone(&dao_journal),
            });

        // services
        let service: Arc<dyn PouetService> = Arc::new(PouetServiceImpl {});

        let rules: Arc<dyn PouetRules> = Arc::new(RulesImpl {
            store: store.clone(),
        });

        let engine: Arc<Engine<PouetStates, PouetCommands, PouetEvents>> = Arc::new(Engine {
            handlers: vec![CommandHandler::Create(Box::new(PouetCreateHandler {
                rules: rules.clone(),
            }))],
            reducer: RegexWordReducer::new().underlying,
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
