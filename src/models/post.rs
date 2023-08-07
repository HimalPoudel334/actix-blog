use super::user::User;
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Identifiable, Associations, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_on: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    title: String,
    content: String,
    created_on: String,
    user_id: i32,
}

impl NewPost {
    pub fn new(title: String, content: String, user_id: i32) -> Self {
        Self {
            title,
            content,
            user_id,
            created_on: Utc::now().to_string(),
        }
    }
}
