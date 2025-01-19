pub mod rules;

use crate::core::{{aggregate_name}}::services::{{aggregate_name | capitalize}}Service;
use async_trait::async_trait;

pub struct {{aggregate_name | capitalize}}ServiceImpl {}

#[async_trait]
impl {{aggregate_name | capitalize}}Service for {{aggregate_name | capitalize}}ServiceImpl {}
