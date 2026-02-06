use chrono::{DateTime, TimeZone, Utc};
use sqlx::types::time::PrimitiveDateTime;

pub fn convert_primitive_to_datetime_utc(primitive: &PrimitiveDateTime) -> DateTime<Utc> {
    let offset = primitive.assume_utc();

    let secs = offset.unix_timestamp();
    let nanos = offset.nanosecond();

    let datetime = Utc.timestamp_opt(secs, nanos as u32).unwrap();

    datetime
}
