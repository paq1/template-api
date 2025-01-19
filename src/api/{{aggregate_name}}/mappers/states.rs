use crate::api::{{aggregate_name}}::{{aggregate_name}}_dbo::{{aggregate_name | capitalize}}DboState;
use crate::core::{{aggregate_name}}::data::states::{{aggregate_name}}::{{aggregate_name | capitalize}};
use crate::core::{{aggregate_name}}::data::states::{{aggregate_name | capitalize}}States;
use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::CanTransform;


impl CanTransform<{{aggregate_name | capitalize}}States> for {{aggregate_name | capitalize}}DboState {
    fn transform_into_other(&self) -> {{aggregate_name | capitalize}}States {
        self.clone().into()
    }
}

impl CanTransform<{{aggregate_name | capitalize}}DboState> for {{aggregate_name | capitalize}}States {
    fn transform_into_other(&self) -> {{aggregate_name | capitalize}}DboState {
        self.clone().into()
    }
}


impl From<{{aggregate_name | capitalize}}DboState> for {{aggregate_name | capitalize}}States {
    fn from(value: {{aggregate_name | capitalize}}DboState) -> Self {
        match value {
            {{aggregate_name | capitalize}}DboState::{{aggregate_name | capitalize}}Dbo { kind, data  } =>
                {{aggregate_name | capitalize}}States::{{aggregate_name | capitalize}}(
                    {{aggregate_name | capitalize}} {
                        kind,
                        data: data.into(),
                    }
                )
        }
    }
}

impl From<{{aggregate_name | capitalize}}States> for {{aggregate_name | capitalize}}DboState {
    fn from(value: {{aggregate_name | capitalize}}States) -> Self {
        match value {
            {{aggregate_name | capitalize}}States::{{aggregate_name | capitalize}}(data) => {
                {{aggregate_name | capitalize}}DboState::{{aggregate_name | capitalize}}Dbo {
                    kind: data.kind,
                    data: data.data.into(),
                }
            }
        }
    }
}
