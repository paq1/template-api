use chrono::{DateTime, Timelike, Utc};
use chrono_tz::Europe::Paris;
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};

pub fn _give_date_time_with_hours(hour: u32, context: &Context) -> ResultErr<DateTime<Utc>> {
    let paris_timezone = context.now.with_timezone(&Paris);

    paris_timezone
        .with_hour(hour)
        .and_then(|d| d.with_minute(0))
        .and_then(|d| d.with_second(0))
        .ok_or(Error::Simple("Création date time impossible".to_string()))
        .map(|date| date.with_timezone(&Utc))
}

pub fn _ctx_is_after_datetime(datetime: &DateTime<Utc>, context: &Context) -> ResultErr<bool> {
    context
        .now
        .with_hour(context.now.hour())
        .ok_or(Error::Simple("Création date time impossible".to_string()))
        .map(|sanitize_date| sanitize_date < *datetime)
}
