use super::dto::UserDto;
use super::repo::UserRepo;
use bcrypt::{DEFAULT_COST, hash};
use std::env;

pub struct UserService {
    repo: UserRepo,
}

impl UserService {
    pub async fn get_all(&self) -> Result<String, sqlx::Error> {
        self.repo.get_all().await
    }

    pub async fn get_all_paginated(
        &self,
        top: Option<i64>,
        skip: Option<i64>,
    ) -> Result<String, sqlx::Error> {
        self.repo.get_all_paginated(top, skip).await
    }
    pub fn new(repo: UserRepo) -> Self {
        Self { repo }
    }

    pub fn respond(&self) -> String {
        "User".to_string()
    }

    pub async fn create_user(&self, mut user: UserDto) -> Result<(), sqlx::Error> {
        // Hash the password before saving
        let cost = env::var("BCRYPT_COST")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(DEFAULT_COST);
        let hashed = hash(&user.password, cost).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        user.password = hashed;
        self.repo.create(user).await.map(|_| ())
    }
}
