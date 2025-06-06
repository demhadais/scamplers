use diesel::{pg::Pg, prelude::*, sql_types};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

pub(super) type BoxedDieselExpression<'a, QuerySource> =
    Box<dyn BoxableExpression<QuerySource, Pg, SqlType = sql_types::Bool> + 'a>;

pub(super) trait NewBoxedDieselExpression<'a, QuerySource> {
    fn new_expression() -> DieselExpressionBuilder<'a, QuerySource>;
}

pub(super) struct DieselExpressionBuilder<'a, QuerySource>(
    Option<BoxedDieselExpression<'a, QuerySource>>,
);

impl<'a, QuerySource: 'a> NewBoxedDieselExpression<'a, QuerySource>
    for BoxedDieselExpression<'_, QuerySource>
{
    fn new_expression() -> DieselExpressionBuilder<'a, QuerySource> {
        DieselExpressionBuilder::new()
    }
}

impl<'a, QuerySource: 'a> DieselExpressionBuilder<'a, QuerySource> {
    pub fn new() -> Self {
        Self(None)
    }

    fn from_query<Q>(query: Q) -> Self
    where
        Q: BoxableExpression<QuerySource, Pg, SqlType = sql_types::Bool> + 'a,
    {
        Self(Some(Box::new(query)))
    }

    pub fn with_condition<Q>(self, other: Q) -> Self
    where
        Q: BoxableExpression<QuerySource, Pg, SqlType = sql_types::Bool> + 'a,
    {
        let Self(Some(query)) = self else {
            return Self::from_query(other);
        };

        let other: BoxedDieselExpression<QuerySource> = Box::new(other);

        Self::from_query(query.and(other))
    }

    pub fn build(self) -> Option<BoxedDieselExpression<'a, QuerySource>> {
        let Self(inner) = self;

        inner
    }
}

pub(super) trait AsIlike {
    fn as_ilike(&self) -> String;
}

impl AsIlike for &str {
    fn as_ilike(&self) -> String {
        format!("%{self}%")
    }
}

impl AsIlike for String {
    fn as_ilike(&self) -> String {
        self.as_str().as_ilike()
    }
}

/// # Errors
pub async fn set_transaction_user(
    user_id: &Uuid,
    db_conn: &mut AsyncPgConnection,
) -> super::error::Result<()> {
    diesel::sql_query(format!(r#"set local role "{user_id}""#))
        .execute(db_conn)
        .await?;

    Ok(())
}
