pub struct UserRepo;
use serde_json::Value;
use sqlx::Row;
use sqlx::postgres::PgRow;

use super::dto::UserDto;
use crate::db::{self, DbParam};
use crate::util::pagination::build_paginated_json_query;

impl UserRepo {
    pub async fn get_all(&self) -> Result<String, sqlx::Error> {
        self.get_all_paginated(None, None).await
    }

    pub async fn get_all_paginated(
        &self,
        top: Option<i64>,
        skip: Option<i64>,
    ) -> Result<String, sqlx::Error> {
        // Use a CTE to fetch paginated data and total count in one query
        let pagination = build_paginated_json_query(
            "USER",
            "id, username",
            "'id', id, 'username', username",
            None,
            vec![],
            top,
            skip,
        );

        let rows = db::query(&pagination.sql, pagination.params).await?;

        let (users_json, total_count) = if let Some(row) = rows.get(0) {
            let users_json = row
                .try_get::<Value, _>("data_json")
                .unwrap_or(Value::Array(vec![]));
            let total_count = row.try_get::<i64, _>("total").unwrap_or(0);
            (users_json, total_count)
        } else {
            (Value::Array(vec![]), 0)
        };

        let page = (pagination.skip / pagination.top) + 1;
        let total_pages = if pagination.top > 0 {
            ((total_count + pagination.top - 1) / pagination.top)
        } else {
            1
        };

        let mut result = serde_json::Map::new();
        result.insert("page".to_string(), Value::Number(page.into()));
        result.insert("total_pages".to_string(), Value::Number(total_pages.into()));
        result.insert("data".to_string(), users_json);

        Ok(Value::Object(result).to_string())
    }

    pub fn new() -> Self {
        Self
    }

    pub async fn create(&self, user: UserDto) -> Result<Vec<PgRow>, sqlx::Error> {
        let res = db::query(
            "INSERT INTO \"USER\" (username, password) VALUES ($1, $2) RETURNING id, username, password",
            vec![
                DbParam::Text(user.username),
                DbParam::Text(user.password),
            ],
        )
        .await;

        res
    }
}
