use std::sync::Arc;
use crate::core::pouet::repositories::CustomPouetRepository;
use crate::core::pouet::services::rules::PouetRules;

pub struct RulesImpl {
    pub store: Arc<dyn CustomPouetRepository>
}

impl PouetRules for RulesImpl {}