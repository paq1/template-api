pub mod rules;

use crate::core::pouet::services::PouetService;
use async_trait::async_trait;

pub struct PouetServiceImpl {}

#[async_trait]
impl PouetService for PouetServiceImpl {}
