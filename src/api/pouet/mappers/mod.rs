use crate::api::pouet::pouet_dbo::PouetDataDbo;
use crate::core::pouet::data::pouet_data::PouetData;

pub mod states;
pub mod events;

impl From<PouetDataDbo> for PouetData {
    fn from(value: PouetDataDbo) -> Self {
        Self {
            message: value.message,
        }
    }
}

impl From<PouetData> for PouetDataDbo {
    fn from(value: PouetData) -> Self {
        Self {
            message: value.message,
        }
    }
}

