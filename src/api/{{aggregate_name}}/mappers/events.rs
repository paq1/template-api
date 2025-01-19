use crate::api::{{aggregate_name}}::{{aggregate_name}}_dbo::{
  {{aggregate_name | capitalize}}CreatedDbo,
  {{aggregate_name | capitalize}}DataDbo,
  {{aggregate_name | capitalize}}DboEvent
};
use crate::core::{{aggregate_name}}::data::events::{
  {{aggregate_name | capitalize}}Created,
  {{aggregate_name | capitalize}}Events
};
use framework_cqrs_lib::cqrs::core::data::EntityEvent;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EventDBO;

impl From<{{aggregate_name | capitalize}}DboEvent> for {{aggregate_name | capitalize}}Events {
    fn from(value: {{aggregate_name | capitalize}}DboEvent) -> Self {
        match value {
            {{aggregate_name | capitalize}}DboEvent::Created(event_dbo) =>
                {{aggregate_name | capitalize}}Events::Created({{aggregate_name | capitalize}}Created {
                    by: event_dbo.by,
                    at: event_dbo.at,
                    data: event_dbo.data.into(),
                }),
        }
    }
}

pub fn from_{{aggregate_name}}_event_dbo_to_event(dbo: EventDBO<{{aggregate_name | capitalize}}DboEvent, String>) -> EntityEvent<{{aggregate_name | capitalize}}Events, String> {
    EntityEvent {
        entity_id: dbo.entity_id,
        data: dbo.data.into(),
        event_id: dbo.event_id,
    }
}

pub fn from_{{aggregate_name}}_event_to_dbo(dbo: EntityEvent<{{aggregate_name | capitalize}}Events, String>) -> EventDBO<{{aggregate_name | capitalize}}DboEvent, String> {
    EventDBO {
        id_mongo: None,
        version: None,
        entity_id: dbo.entity_id,
        data: dbo.data.into(),
        event_id: dbo.event_id,
    }
}

impl From<{{aggregate_name | capitalize}}Events> for {{aggregate_name | capitalize}}DboEvent {
    fn from(value: {{aggregate_name | capitalize}}Events) -> Self {
        match value {
            {{aggregate_name | capitalize}}Events::Created(event) => {{aggregate_name | capitalize}}DboEvent::Created(
                {{aggregate_name | capitalize}}CreatedDbo {
                    by: event.by,
                    at: event.at,
                    data: {{aggregate_name | capitalize}}DataDbo {
                        message: event.data.message
                    },
                }
            )
        }
    }
}

