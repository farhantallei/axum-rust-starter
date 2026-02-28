use sqlx::{Postgres, QueryBuilder};

use crate::{
    infrastructure::sql::{filter::Filter, order::OrderBy},
    modules::{
        user::domain::spec::{UserFilter, UserJoin, UserOrder},
        user_role::persistence::query::UserRoleQuery,
    },
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
        qb.push("SELECT COUNT(*)");

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
        qb.push(alias);
        qb.push(".");
        match cond {
            UserFilter::NameLike(value) => {
                qb.push("name ILIKE '%' || ");
                qb.push_bind(value.to_string());
                qb.push(" || '%'");
            }

            UserFilter::Email(value) => {
                qb.push("email = ");
                qb.push_bind(value.to_string());
            }

            UserFilter::IsActive(actived) => {
                qb.push("status = ");
                qb.push_bind(*actived);
            }

            UserFilter::IsDeleted(deleted) => {
                if *deleted {
                    qb.push("deleted_at IS NOT NULL");
                } else {
                    qb.push("deleted_at IS NULL");
                }
            }
        }
    }

    fn order_fragment<'a>(qb: &mut QueryBuilder<'a, Postgres>, col: &UserOrder, alias: &str) {
        qb.push(alias);
        qb.push(".");
        qb.push(match col {
            UserOrder::Id => "id",
            UserOrder::Name => "name",
            UserOrder::Email => "email",
            UserOrder::CreatedAt => "created_at",
        });
    }
}
