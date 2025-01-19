use crate::api::pouet::pouet_dbo::{PouetCreatedDbo, PouetDataDbo, PouetDboEvent};
use crate::core::pouet::data::events::{PouetCreated, PouetEvents};
use framework_cqrs_lib::cqrs::core::data::EntityEvent;
use framework_cqrs_lib::cqrs::infra::daos::dbos::EventDBO;

impl From<PouetDboEvent> for PouetEvents {
    fn from(value: PouetDboEvent) -> Self {
        match value {
            PouetDboEvent::Created(event_dbo) =>
                PouetEvents::Created(PouetCreated {
                    by: event_dbo.by,
                    at: event_dbo.at,
                    data: event_dbo.data.into(),
                }),
        }
    }
}

pub fn from_pouet_event_dbo_to_event(dbo: EventDBO<PouetDboEvent, String>) -> EntityEvent<PouetEvents, String> {
    EntityEvent {
        entity_id: dbo.entity_id,
        data: dbo.data.into(),
        event_id: dbo.event_id,
    }
}

pub fn from_pouet_event_to_dbo(dbo: EntityEvent<PouetEvents, String>) -> EventDBO<PouetDboEvent, String> {
    EventDBO {
        id_mongo: None,
        version: None,
        entity_id: dbo.entity_id,
        data: dbo.data.into(),
        event_id: dbo.event_id,
    }
}

impl From<PouetEvents> for PouetDboEvent {
    fn from(value: PouetEvents) -> Self {
        match value {
            PouetEvents::Created(event) => PouetDboEvent::Created(
                PouetCreatedDbo {
                    by: event.by,
                    at: event.at,
                    data: PouetDataDbo {
                        message: event.data.message
                    },
                }
            )
        }
    }
}

