use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub birth_date: NaiveDate,
    pub custom_data: CustomData,
    pub create_at: Option<DateTime<Utc>>,
    pub update_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(name: String, birth_date_ymd: (i32, u32, u32)) -> Self {
        let (year, month, day) = birth_date_ymd;
        let id = uuid::Uuid::parse_str("9c8016c3-4a21-4dfd-9b7c-36b303be4fc5").unwrap();
        Self {
            id,
            name,
            birth_date: NaiveDate::from_ymd(year, month, day),
            custom_data: CustomData { random: 1 },
            create_at: Some(Utc::now()),
            update_at: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomData {
    pub random: u32,
}
