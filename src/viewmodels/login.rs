use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct LoginVM {
    pub username: String,
    pub password: String,
}
