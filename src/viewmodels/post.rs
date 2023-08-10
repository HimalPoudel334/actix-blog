use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::models::post::Post;

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
    pub fn from(post: &Post) -> Self {
        Self {
            id: post.id,
            title: post.title.to_owned(),
            content: post.content.to_owned(),
            created_on: post.created_on.to_owned(),
            user_id: post.user_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
pub struct UsersPostsVM {
    pub uid: i32,
    pub username: String,
    pub profile_image: String,
    pub pid: i32,
    pub title: String,
    pub content: String,
    pub created_on: String,
    pub user_id: i32,
}
