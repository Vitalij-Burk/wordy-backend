use chrono::{DateTime, Utc};
use sqlx::types::time::{OffsetDateTime, PrimitiveDateTime};

pub fn convert_datetime_utc_to_primitive(datetime: &DateTime<Utc>) -> PrimitiveDateTime {
    let offset = OffsetDateTime::from_unix_timestamp(datetime.timestamp() as i64).unwrap();

    let primitive = PrimitiveDateTime::new(offset.date(), offset.time());

    primitive
}
