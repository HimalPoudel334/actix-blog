use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::models::user::User;

use super::post::PostVM;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateVM {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub profile_img: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserVM {
    pub id: i32,
    pub username: String,
    pub profile_img: String,
}

impl UserVM {
    pub fn from(user_model: &User) -> Self {
        Self {
            id: user_model.id,
            username: user_model.username.to_owned(),
            profile_img: user_model.profile_image.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileVM {
    pub user: UserVM,
    pub posts: Vec<PostVM>,
}

#[derive(Serialize, Deserialize)]
pub struct UserTimeZone {
    pub timezone: String,
}
