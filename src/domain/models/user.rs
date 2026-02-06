use chrono::{DateTime, Utc};
use heck::ToTitleCase;
use uuid::Uuid;

use crate::domain::types::ID;

#[derive(Debug, Clone)]
pub struct User {
    pub id: ID,
    pub hashed_password: String,

    pub key: String,
    pub name: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        key: String,
        name: String,
        hashed_password: String,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        let id = Uuid::new_v4();

        let created_time: DateTime<Utc>;
        let updated_time: DateTime<Utc>;

        match created_at {
            None => {
                created_time = Utc::now();
            }
            Some(time) => {
                created_time = time;
            }
        }

        match updated_at {
            None => {
                updated_time = Utc::now();
            }
            Some(time) => {
                updated_time = time;
            }
        }

        Self {
            id: id,
            hashed_password: hashed_password,
            key: key,
            name: name.to_title_case(),
            created_at: created_time,
            updated_at: updated_time,
        }
    }

    pub fn new_simple(key: String, name: String, hashed_password: String) -> Self {
        let id = Uuid::new_v4();

        let created_time = Utc::now();
        let updated_time = Utc::now();

        Self {
            id: id,
            hashed_password: hashed_password,
            key: key,
            name: name.to_title_case(),
            created_at: created_time,
            updated_at: updated_time,
        }
    }

    pub fn update(&mut self, key: Option<String>, name: Option<String>) -> &mut Self {
        if let Some(key) = key {
            self.key = key;
        }

        if let Some(name) = name {
            self.name = name;
        }

        self.updated_at = Utc::now();

        self
    }
}
