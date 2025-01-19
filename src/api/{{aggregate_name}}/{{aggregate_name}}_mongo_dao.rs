use crate::api::{{aggregate_name}}::{{aggregate_name}}_dbo::{
  {{aggregate_name | capitalize}}DboEvent,
  {{aggregate_name | capitalize}}DboState
};
use framework_cqrs_lib::cqrs::infra::daos::mongo_entity_dao::{EntityMongoDAO, EventMongoDAO};

pub type {{aggregate_name | capitalize}}MongoDAO = EntityMongoDAO<{{aggregate_name | capitalize}}DboState>;
pub type {{aggregate_name | capitalize}}EventMongoDAO = EventMongoDAO<{{aggregate_name | capitalize}}DboEvent>;