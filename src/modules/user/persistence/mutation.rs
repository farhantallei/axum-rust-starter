use sqlx::{Postgres, QueryBuilder};

use crate::modules::user::domain::model::{UpdateUserPayload, UserPayload};

pub struct UserMutation;

impl UserMutation {
    pub const TABLE: &'static str = "users";

    pub fn insert<'a>(qb: &mut QueryBuilder<'a, Postgres>, payload: &'a UserPayload) {
        qb.push(format!("INSERT INTO {}", Self::TABLE));
        qb.push(
            " (username, password, name, email) \
            VALUES (",
        );
        let mut sep = qb.separated(", ");
        sep.push_bind(&payload.username);
        sep.push_bind(&payload.password);
        sep.push_bind(&payload.name);
        sep.push_bind(&payload.email);
        qb.push(
            ") RETURNING id, username, password, name, email, \
            status, created_at, updated_at, deleted_at",
        );
    }

    pub fn update<'a>(
        qb: &mut QueryBuilder<'a, Postgres>,
        id: i32,
        payload: &'a UpdateUserPayload,
    ) {
        qb.push(format!("UPDATE {} SET ", Self::TABLE));

        let mut sep = qb.separated(", ");

        if let Some(v) = &payload.username {
            sep.push("username = ");
            sep.push_bind_unseparated(v.as_str());
        }
        if let Some(v) = &payload.name {
            sep.push("name = ");
            sep.push_bind_unseparated(v.as_str());
        }

        sep.push("updated_at = NOW()");

        qb.push(" WHERE id = ");
        qb.push_bind(id);
        qb.push(
            " AND deleted_at is null \
            RETURNING id, username, password, name, email, \
            status, created_at, updated_at, deleted_at",
        );
    }

    pub fn delete<'a>(qb: &mut QueryBuilder<'a, Postgres>, id: i32) {
        qb.push(format!("UPDATE {} SET", Self::TABLE));
        qb.push(" deleted_at = NOW() WHERE id = ");
        qb.push_bind(id);
    }
}
