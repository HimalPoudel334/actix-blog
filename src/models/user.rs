use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub profile_image: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    username: String,
    password: String,
    profile_image: String,
}

impl NewUser {
    pub fn new(username: String, password: String, profile_image: Option<String>) -> Self {
        Self {
            username,
            password,
            profile_image: match profile_image {
                Some(img) => img,
                None => "/static/images/default.jpg".to_string(),
            },
        }
    }
}
