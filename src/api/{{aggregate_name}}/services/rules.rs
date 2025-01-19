use std::sync::Arc;
use crate::core::{{aggregate_name}}::repositories::Custom{{aggregate_name | capitalize}}Repository;
use crate::core::{{aggregate_name}}::services::rules::{{aggregate_name | capitalize}}Rules;

pub struct RulesImpl {
    pub store: Arc<dyn Custom{{aggregate_name | capitalize}}Repository>
}

impl {{aggregate_name | capitalize}}Rules for RulesImpl {}