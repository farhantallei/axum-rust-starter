use sqlx::{Postgres, QueryBuilder};

pub trait ApplyFilter {
    fn apply<'a>(&self, qb: &mut QueryBuilder<'a, Postgres>);
}

#[derive(Clone)]
pub enum Filter<C> {
    Condition(C),
    And(Vec<Filter<C>>),
    Or(Vec<Filter<C>>),
}

impl<C: ApplyFilter> Filter<C> {
    pub fn apply<'a>(&self, qb: &mut QueryBuilder<'a, Postgres>) {
        qb.push(" WHERE ");
        self.apply_inner(qb);
    }

    fn apply_inner<'a>(&self, qb: &mut QueryBuilder<'a, Postgres>) {
        match self {
            Filter::Condition(cond) => {
                cond.apply(qb);
            }

            Filter::And(children) => {
                qb.push("(");
                for (i, c) in children.iter().enumerate() {
                    if i > 0 {
                        qb.push(" AND ");
                    }
                    c.apply_inner(qb);
                }
                qb.push(")");
            }

            Filter::Or(children) => {
                qb.push("(");
                for (i, c) in children.iter().enumerate() {
                    if i > 0 {
                        qb.push(" OR ");
                    }
                    c.apply_inner(qb);
                }
                qb.push(")");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Execute;

    #[derive(Clone)]
    struct MockCond(&'static str);

    impl ApplyFilter for MockCond {
        fn apply<'a>(&self, qb: &mut QueryBuilder<'a, Postgres>) {
            qb.push(self.0);
        }
    }

    fn build_sql(filter: &Filter<MockCond>) -> String {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM test");
        filter.apply(&mut qb);
        qb.build().sql().to_string()
    }

    #[test]
    fn single_condition() {
        let filter = Filter::Condition(MockCond("a = 1"));

        let sql = build_sql(&filter);

        assert_eq!(sql, "SELECT * FROM test WHERE a = 1");
    }

    #[test]
    fn and_filter() {
        let filter = Filter::And(vec![
            Filter::Condition(MockCond("a = 1")),
            Filter::Condition(MockCond("b = 2")),
        ]);

        let sql = build_sql(&filter);

        assert_eq!(sql, "SELECT * FROM test WHERE (a = 1 AND b = 2)");
    }

    #[test]
    fn or_filter() {
        let filter = Filter::Or(vec![
            Filter::Condition(MockCond("a = 1")),
            Filter::Condition(MockCond("b = 2")),
        ]);

        let sql = build_sql(&filter);

        assert_eq!(sql, "SELECT * FROM test WHERE (a = 1 OR b = 2)");
    }

    #[test]
    fn nested_and_or() {
        let filter = Filter::And(vec![
            Filter::Condition(MockCond("a = 1")),
            Filter::Or(vec![
                Filter::Condition(MockCond("b = 2")),
                Filter::Condition(MockCond("c = 3")),
            ]),
        ]);

        let sql = build_sql(&filter);

        assert_eq!(sql, "SELECT * FROM test WHERE (a = 1 AND (b = 2 OR c = 3))");
    }

    #[test]
    fn no_double_where() {
        let filter = Filter::And(vec![Filter::Condition(MockCond("a = 1"))]);

        let sql = build_sql(&filter);

        assert_eq!(sql.matches("WHERE").count(), 1);
    }

    #[test]
    fn deep_nesting() {
        let filter = Filter::And(vec![
            Filter::And(vec![
                Filter::Condition(MockCond("a = 1")),
                Filter::Condition(MockCond("b = 2")),
            ]),
            Filter::Condition(MockCond("c = 3")),
        ]);

        let sql = build_sql(&filter);

        assert_eq!(
            sql,
            "SELECT * FROM test WHERE ((a = 1 AND b = 2) AND c = 3)"
        );
    }
}
