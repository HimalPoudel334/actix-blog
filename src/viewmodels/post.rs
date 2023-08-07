use chrono::{DateTime, Utc};
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Serialize, Deserialize)]
pub struct PostCreateVM {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct PostVM {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_on: String,
    pub user_id: i32,
}

impl PostVM {
    pub fn new(id: i32, title: String, content: String, created_on: String, user_id: i32) -> Self {
        Self {
            id,
            title,
            content,
            created_on: humanize(created_on),
            user_id,
        }
    }
}

pub fn humanize(created_on: String) -> String {
    // let dt = chrono::Utc::now() - chrono::Duration::minutes(58);
    let parsed_dt = created_on.parse::<DateTime<Utc>>().unwrap();

    let datetime_diff = chrono::Utc::now() - parsed_dt;
    chrono_humanize::HumanTime::from(datetime_diff).to_string()
}
