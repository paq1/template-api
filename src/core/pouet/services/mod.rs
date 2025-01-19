pub mod rules;

use async_trait::async_trait;

#[async_trait]
pub trait PouetService: Send + Sync {

}
