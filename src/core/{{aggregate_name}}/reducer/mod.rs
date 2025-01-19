use crate::core::{{aggregate_name}}::data::events::{{aggregate_name | capitalize}}Events;
use crate::core::{{aggregate_name}}::data::states::{{aggregate_name | capitalize}}States;
use framework_cqrs_lib::cqrs::core::reducer::Reducer;

pub struct RegexWordReducer {
    pub underlying: Reducer<{{aggregate_name | capitalize}}Events, {{aggregate_name | capitalize}}States>,
}

impl RegexWordReducer {
    pub fn new() -> Self {
        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    match current {
                        Some(current_state) => current_state.reduce_state(event),
                        None => {{aggregate_name | capitalize}}States::reduce_state_from_empty(event)
                    }
                }
            }
        }
    }
}