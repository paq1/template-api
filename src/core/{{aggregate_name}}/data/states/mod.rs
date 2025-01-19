use crate::core::{{aggregate_name}}::data::events::{{aggregate_name | capitalize}}Events;
use crate::core::{{aggregate_name}}::data::events::{{aggregate_name | capitalize}}Events::Created;
use crate::core::{{aggregate_name}}::data::states::{{aggregate_name}}::{{aggregate_name | capitalize}};
use crate::models::{{aggregate_name}}::views::{{aggregate_name | capitalize}}ViewState;
use framework_cqrs_lib::cqrs::models::jsonapi::{CanBeView, CanGetTypee};

pub mod {{aggregate_name}};

#[derive(Clone, Debug)]
pub enum {{aggregate_name | capitalize}}States {
    RegexWord({{aggregate_name | capitalize}}),
}

impl {{aggregate_name | capitalize}}States {
    pub fn reduce_state(&self, event: {{aggregate_name | capitalize}}Events) -> Option<{{aggregate_name | capitalize}}States> {
        match self {
            {{aggregate_name | capitalize}}States::RegexWord(c) => c.reduce_state(event),
        }
    }

    pub fn reduce_state_from_empty(event: {{aggregate_name | capitalize}}Events) -> Option<{{aggregate_name | capitalize}}States> {
        match event {
            Created(data) =>
                Some(
                    {{aggregate_name | capitalize}}States::RegexWord(
                        {{aggregate_name | capitalize}} {
                            kind: "urn:api:{{aggregate_name}}:{{aggregate_name}}".to_string(),
                            data: data.data,
                        }
                    )
                ),
        }
    }
}

impl CanGetTypee for {{aggregate_name | capitalize}}States {
    fn get_type(&self) -> String {
        match self {
            {{aggregate_name | capitalize}}States::RegexWord(state) => state.get_type(),
        }
    }
}

impl CanBeView<{{aggregate_name | capitalize}}ViewState> for {{aggregate_name | capitalize}}States {
    fn to_view(&self) -> {{aggregate_name | capitalize}}ViewState {
        match self.clone() {
            {{aggregate_name | capitalize}}States::RegexWord(state) =>
                {{aggregate_name | capitalize}}ViewState::Create(state.into()),
        }
    }
}