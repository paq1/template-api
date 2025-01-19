use crate::api::pouet::pouet_dbo::{PouetDboEvent, PouetDboState};
use framework_cqrs_lib::cqrs::infra::daos::mongo_entity_dao::{EntityMongoDAO, EventMongoDAO};

pub type PouetMongoDAO = EntityMongoDAO<PouetDboState>;
pub type PouetEventMongoDAO = EventMongoDAO<PouetDboEvent>;