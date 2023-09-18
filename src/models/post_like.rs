use super::{post::Post, user::User};
use diesel::{
    prelude::{Associations, Identifiable, Insertable, Queryable},
    Selectable,
};

#[derive(Insertable, Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Post))]
#[diesel(table_name = crate::schema::post_likes)]
#[diesel(primary_key(user_id, post_id))]
pub struct PostLike {
    pub user_id: i32,
    pub post_id: i32,
}

impl PostLike {
    pub fn new(user_id: i32, post_id: i32) -> Self {
        Self { user_id, post_id }
    }
}
