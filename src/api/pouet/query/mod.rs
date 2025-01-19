use actix_web::web::Query;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use framework_cqrs_lib::cqrs::core::repositories::filter::Filter;
use framework_cqrs_lib::cqrs::core::repositories::query::PaginationDef;


#[derive(Serialize, Deserialize, IntoParams, Debug, Clone)]
pub struct RegexWordQuery {
    #[serde(rename = "page[number]")]
    pub number: Option<usize>,
    #[serde(rename = "page[size]")]
    pub size: Option<usize>,
}

pub fn from_pouet_query_to_query_repo(q: Query<RegexWordQuery>) -> framework_cqrs_lib::cqrs::core::repositories::query::Query {
    let size = q.size.unwrap_or(10);
    let number = q.number.unwrap_or(0);

    framework_cqrs_lib::cqrs::core::repositories::query::Query {
        pagination: PaginationDef {
            page_number: number,
            page_size: size,
        },
        filter: Filter::None,
    }
}