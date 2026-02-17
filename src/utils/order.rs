use sqlx::{Postgres, QueryBuilder};

pub trait ApplyOrder {
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, Postgres>);
}

#[derive(Clone)]
pub enum Order<C> {
    Asc(C),
    Desc(C),
}

#[derive(Clone, Default)]
pub struct OrderBy<C>(pub Vec<Order<C>>);

impl<C: ApplyOrder> OrderBy<C> {
    pub fn apply<'a>(&self, qb: &mut QueryBuilder<'a, Postgres>) {
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
                    col.apply(qb);
                    qb.push(" ASC");
                }
                Order::Desc(col) => {
                    col.apply(qb);
                    qb.push(" DESC");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Execute;

    #[derive(Clone)]
    struct MockOrder(&'static str);

    impl ApplyOrder for MockOrder {
        fn apply<'a>(&self, qb: &mut QueryBuilder<'a, Postgres>) {
            qb.push(self.0);
        }
    }

    #[test]
    fn order_by_multiple_columns() {
        let order = OrderBy(vec![
            Order::Asc(MockOrder("a")),
            Order::Desc(MockOrder("b")),
        ]);

        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM test");
        order.apply(&mut qb);

        let sql = qb.build().sql();

        assert_eq!(sql, "SELECT * FROM test ORDER BY a ASC, b DESC");
    }
}
