use crate::core::pouet::data::events::PouetEvents;
use crate::core::pouet::data::pouet_data::PouetData;
use crate::core::pouet::data::states::PouetStates;
use crate::models::pouet::views::{PouetDataView, PouetView};
use framework_cqrs_lib::cqrs::models::jsonapi::CanGetTypee;

#[derive(Clone, Debug)]
pub struct Pouet {
    pub kind: String,
    pub data: PouetData,
}

impl Pouet {
    pub fn reduce_state(&self, event: PouetEvents) -> Option<PouetStates> {
        // on accept pas d'evenement apres la creation
        match event {
            _ => None // illegal transition
        }
    }
}

impl CanGetTypee for Pouet {
    fn get_type(&self) -> String {
        "urn:api:{{aggregate_name}}:{{aggregate_name}}".to_string()
    }
}

impl From<Pouet> for PouetView {
    fn from(value: Pouet) -> Self {
        PouetView {
            data: value.data.into(),
        }
    }
}

impl From<PouetData> for PouetDataView {
    fn from(value: PouetData) -> Self {
        Self {
            message: value.message,
        }
    }
}

