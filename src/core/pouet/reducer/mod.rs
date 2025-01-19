use crate::core::pouet::data::events::PouetEvents;
use crate::core::pouet::data::states::PouetStates;
use framework_cqrs_lib::cqrs::core::reducer::Reducer;

pub struct RegexWordReducer {
    pub underlying: Reducer<PouetEvents, PouetStates>,
}

impl RegexWordReducer {
    pub fn new() -> Self {
        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    match current {
                        Some(current_state) => current_state.reduce_state(event),
                        None => PouetStates::reduce_state_from_empty(event)
                    }
                }
            }
        }
    }
}