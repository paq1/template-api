use chrono::{DateTime, Utc};

use crate::core::pouet::data::pouet_data::PouetData;
use crate::models::pouet::views::{PouetCreatedView, PouetDataView, PouetViewEvent};
use framework_cqrs_lib::cqrs::models::jsonapi::CanBeView;

impl CanBeView<PouetViewEvent> for PouetEvents {
    fn to_view(&self) -> PouetViewEvent {
        match self.clone() {
            PouetEvents::Created(c) => PouetViewEvent::Created(PouetCreatedView {
                data: PouetDataView {
                    message: c.data.message,
                },
                by: c.by,
                at: c.at
            }),
        }
    }
}


#[derive(Clone)]
pub enum PouetEvents {
    Created(PouetCreated),
}

#[derive(Clone)]
pub struct PouetCreated {
    pub by: String,
    pub at: DateTime<Utc>,
    pub data: PouetData,
}
