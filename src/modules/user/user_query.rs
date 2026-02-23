use sqlx::{Postgres, QueryBuilder};

use crate::{
    modules::{
        user::user_spec::{UserFilter, UserJoin, UserOrder},
        user_role::user_role_query::UserRoleQuery,
    },
    utils::{filter::Filter, order::OrderBy},
};

pub struct UserQuery;

impl UserQuery {
    pub const TABLE: &'static str = "users";
    const BASE_ALIAS: &'static str = "u";
    const ROLE_ALIAS: &'static str = "r";

    pub fn select<'a>(qb: &mut QueryBuilder<'a, Postgres>, joins: &[UserJoin]) {
        // SELECT
        qb.push(format!("SELECT {}.*", Self::BASE_ALIAS));

        for j in joins {
            Self::select_fragment(qb, j);
        }

        // FROM
        qb.push(format!(" FROM {} {}", Self::TABLE, Self::BASE_ALIAS));

        // JOIN
        for j in joins {
            Self::join_fragment(qb, j);
        }
    }

    pub fn count<'a>(qb: &mut QueryBuilder<'a, Postgres>, joins: &[UserJoin]) {
        // SELECT
        qb.push(format!("SELECT COUNT({}.*)", Self::BASE_ALIAS));

        // FROM
        qb.push(format!(" FROM {} {}", Self::TABLE, Self::BASE_ALIAS));

        // JOIN
        for j in joins {
            Self::join_fragment(qb, j);
        }
    }

    pub fn filter<'a>(qb: &mut QueryBuilder<'a, Postgres>, filters: &[UserFilter]) {
        if filters.is_empty() {
            return;
        }

        // WHERE
        let filter_tree = Filter::And(filters.iter().cloned().map(Filter::Condition).collect());

        filter_tree.apply(qb, Self::BASE_ALIAS, &|cond, qb, alias| {
            Self::filter_fragment(qb, cond, alias);
        });
    }

    pub fn order<'a>(qb: &mut QueryBuilder<'a, Postgres>, orders: &OrderBy<UserOrder>) {
        // ORDER
        orders.apply(qb, Self::BASE_ALIAS, &|col, qb, alias| {
            Self::order_fragment(qb, col, alias);
        });
    }

    fn select_fragment<'a>(qb: &mut QueryBuilder<'a, Postgres>, join: &UserJoin) {
        match join {
            UserJoin::UserRole => {
                qb.push(format!(", to_jsonb({}.*) AS role", Self::ROLE_ALIAS));
            }
        }
    }

    fn join_fragment<'a>(qb: &mut QueryBuilder<'a, Postgres>, join: &UserJoin) {
        match join {
            UserJoin::UserRole => {
                qb.push(format!(
                    " LEFT JOIN {} {} ON {}.id = {}.role_id ",
                    UserRoleQuery::TABLE,
                    Self::ROLE_ALIAS,
                    Self::ROLE_ALIAS,
                    Self::BASE_ALIAS
                ));
            }
        }
    }

    fn filter_fragment<'a>(qb: &mut QueryBuilder<'a, Postgres>, cond: &UserFilter, alias: &str) {
        match cond {
            UserFilter::NameLike(value) => {
                qb.push(format!("{}.name ILIKE ", alias));
                qb.push_bind(format!("%{}%", value));
            }

            UserFilter::Email(value) => {
                qb.push(format!("{}.email = ", alias));
                qb.push_bind(value.to_string());
            }

            UserFilter::IsActive(actived) => {
                qb.push(format!("{}.status = ", alias));
                qb.push_bind(*actived);
            }

            UserFilter::IsDeleted(deleted) => {
                if *deleted {
                    qb.push(format!(" {}.deleted_at IS NOT NULL", alias));
                } else {
                    qb.push(format!(" {}.deleted_at IS NULL", alias));
                }
            }
        }
    }

    fn order_fragment<'a>(qb: &mut QueryBuilder<'a, Postgres>, col: &UserOrder, alias: &str) {
        match col {
            UserOrder::Id => {
                qb.push(format!("{}.id", alias));
            }
            UserOrder::Name => {
                qb.push(format!("{}.name", alias));
            }
            UserOrder::Email => {
                qb.push(format!("{}.email", alias));
            }
            UserOrder::CreatedAt => {
                qb.push(format!("{}.created_at", alias));
            }
        }
    }
}
