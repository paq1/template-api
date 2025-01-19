use crate::api::pouet::pouet_dbo::PouetDboState;
use crate::core::pouet::data::states::pouet::Pouet;
use crate::core::pouet::data::states::PouetStates;
use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::CanTransform;


impl CanTransform<PouetStates> for PouetDboState {
    fn transform_into_other(&self) -> PouetStates {
        self.clone().into()
    }
}

impl CanTransform<PouetDboState> for PouetStates {
    fn transform_into_other(&self) -> PouetDboState {
        self.clone().into()
    }
}


impl From<PouetDboState> for PouetStates {
    fn from(value: PouetDboState) -> Self {
        match value {
            PouetDboState::PouetDbo { kind, data  } =>
                PouetStates::RegexWord(
                    Pouet {
                        kind,
                        data: data.into(),
                    }
                )
        }
    }
}

impl From<PouetStates> for PouetDboState {
    fn from(value: PouetStates) -> Self {
        match value {
            PouetStates::RegexWord(data) => {
                PouetDboState::PouetDbo {
                    kind: data.kind,
                    data: data.data.into(),
                }
            }
        }
    }
}
