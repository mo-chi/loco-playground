use serde::{Deserialize, Serialize};

use crate::models::_entities::users;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl UserResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }

    pub fn news(users: Vec<users::Model>) -> Vec<Self> {
        users
            .iter()
            // NOTE: `.map(|user| Self::new(user))` の短縮系
            .map(Self::new)
            .collect::<Vec<Self>>()
    }
}
