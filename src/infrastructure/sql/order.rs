use sqlx::{Postgres, QueryBuilder};

#[derive(Clone)]
pub enum Order<C> {
    Asc(C),
    Desc(C),
}

impl<C> Order<C> {
    pub fn from_str(order: Option<&str>, field: C) -> Self {
        match order {
            Some("asc") => Self::Asc(field),
            _ => Self::Desc(field),
        }
    }
}

#[derive(Clone, Default)]
pub struct OrderBy<C>(pub Vec<Order<C>>);

impl<C> OrderBy<C> {
    pub fn apply<'a, F>(&self, qb: &mut QueryBuilder<'a, Postgres>, alias: &str, render: &F)
    where
        F: Fn(&C, &mut QueryBuilder<'a, Postgres>, &str),
    {
        if self.0.is_empty() {
            return;
        }

        qb.push(" ORDER BY ");

        for (i, order) in self.0.iter().enumerate() {
            if i > 0 {
                qb.push(", ");
            }

            match order {
                Order::Asc(col) => {
                    render(col, qb, alias);
                    qb.push(" ASC");
                }
                Order::Desc(col) => {
                    render(col, qb, alias);
                    qb.push(" DESC");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Execute, Postgres, QueryBuilder};

    #[derive(Clone)]
    struct MockOrder(&'static str);

    #[test]
    fn order_by_multiple_columns() {
        let order = OrderBy(vec![
            Order::Asc(MockOrder("a")),
            Order::Desc(MockOrder("b")),
        ]);

        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM test alias");

        order.apply(&mut qb, "alias", &|col, qb, alias| {
            qb.push(format!("{}.{}", alias, col.0));
        });

        let sql = qb.build().sql();

        assert_eq!(
            sql,
            "SELECT * FROM test alias ORDER BY alias.a ASC, alias.b DESC"
        );
    }

    #[test]
    fn empty_order_should_not_emit_clause() {
        let order: OrderBy<MockOrder> = OrderBy(vec![]);

        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM test alias");

        order.apply(&mut qb, "alias", &|col, qb, alias| {
            qb.push(format!("{}.{}", alias, col.0));
        });

        let sql = qb.build().sql();

        assert_eq!(sql, "SELECT * FROM test alias");
    }
}
