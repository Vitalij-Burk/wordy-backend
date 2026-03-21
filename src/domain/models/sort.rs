use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SortBy {
    Time(SortDirection),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TimeRange {
    From(DateTime<Utc>),
    To(DateTime<Utc>),
    Between(DateTime<Utc>, DateTime<Utc>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FilterBy {
    Time(TimeRange),
    Language {
        target_language: Option<String>,
        source_language: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWordPairsQueryList {
    pub sort_by: Option<SortBy>,
    pub filter_by: Option<Vec<FilterBy>>,
}
