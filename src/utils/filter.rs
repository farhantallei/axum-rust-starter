use sqlx::{Postgres, QueryBuilder};

#[derive(Clone)]
pub enum Filter<C> {
    Condition(C),
    And(Vec<Filter<C>>),
    Or(Vec<Filter<C>>),
}

impl<C> Filter<C> {
    pub fn apply<'a, F>(&self, qb: &mut QueryBuilder<'a, Postgres>, alias: &str, render: &F)
    where
        F: Fn(&C, &mut QueryBuilder<'a, Postgres>, &str),
    {
        qb.push(" WHERE ");
        self.apply_inner(qb, alias, render);
    }

    fn apply_inner<'a, F>(&self, qb: &mut QueryBuilder<'a, Postgres>, alias: &str, render: &F)
    where
        F: Fn(&C, &mut QueryBuilder<'a, Postgres>, &str),
    {
        match self {
            Filter::Condition(cond) => render(cond, qb, alias),

            Filter::And(children) => {
                qb.push("(");
                for (i, c) in children.iter().enumerate() {
                    if i > 0 {
                        qb.push(" AND ");
                    }
                    c.apply_inner(qb, alias, render);
                }
                qb.push(")");
            }

            Filter::Or(children) => {
                qb.push("(");
                for (i, c) in children.iter().enumerate() {
                    if i > 0 {
                        qb.push(" OR ");
                    }
                    c.apply_inner(qb, alias, render);
                }
                qb.push(")");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Execute, Postgres, QueryBuilder};

    #[derive(Clone)]
    struct MockCond(&'static str);

    fn build_sql(filter: &Filter<MockCond>) -> String {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM test alias");

        filter.apply(&mut qb, "alias", &|cond, qb, alias| {
            qb.push(format!("{}.{}", alias, cond.0));
        });

        qb.build().sql().to_string()
    }

    #[test]
    fn single_condition() {
        let filter = Filter::Condition(MockCond("a = 1"));

        let sql = build_sql(&filter);

        assert_eq!(sql, "SELECT * FROM test alias WHERE alias.a = 1");
    }

    #[test]
    fn and_filter() {
        let filter = Filter::And(vec![
            Filter::Condition(MockCond("a = 1")),
            Filter::Condition(MockCond("b = 2")),
        ]);

        let sql = build_sql(&filter);

        assert_eq!(
            sql,
            "SELECT * FROM test alias WHERE (alias.a = 1 AND alias.b = 2)"
        );
    }

    #[test]
    fn or_filter() {
        let filter = Filter::Or(vec![
            Filter::Condition(MockCond("a = 1")),
            Filter::Condition(MockCond("b = 2")),
        ]);

        let sql = build_sql(&filter);

        assert_eq!(
            sql,
            "SELECT * FROM test alias WHERE (alias.a = 1 OR alias.b = 2)"
        );
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

        assert_eq!(
            sql,
            "SELECT * FROM test alias WHERE (alias.a = 1 AND (alias.b = 2 OR alias.c = 3))"
        );
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
            "SELECT * FROM test alias WHERE ((alias.a = 1 AND alias.b = 2) AND alias.c = 3)"
        );
    }
}
