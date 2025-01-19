use crate::core::pouet::data::events::PouetEvents;
use crate::core::pouet::data::events::PouetEvents::Created;
use crate::core::pouet::data::states::pouet::Pouet;
use crate::models::pouet::views::PouetViewState;
use framework_cqrs_lib::cqrs::models::jsonapi::{CanBeView, CanGetTypee};

pub mod pouet;

#[derive(Clone, Debug)]
pub enum PouetStates {
    RegexWord(Pouet),
}

impl PouetStates {
    pub fn reduce_state(&self, event: PouetEvents) -> Option<PouetStates> {
        match self {
            PouetStates::RegexWord(c) => c.reduce_state(event),
        }
    }

    pub fn reduce_state_from_empty(event: PouetEvents) -> Option<PouetStates> {
        match event {
            Created(data) =>
                Some(
                    PouetStates::RegexWord(
                        Pouet {
                            kind: "urn:api:{{aggregate_name}}:{{aggregate_name}}".to_string(),
                            data: data.data,
                        }
                    )
                ),
        }
    }
}

impl CanGetTypee for PouetStates {
    fn get_type(&self) -> String {
        match self {
            PouetStates::RegexWord(state) => state.get_type(),
        }
    }
}

impl CanBeView<PouetViewState> for PouetStates {
    fn to_view(&self) -> PouetViewState {
        match self.clone() {
            PouetStates::RegexWord(state) =>
                PouetViewState::Create(state.into()),
        }
    }
}