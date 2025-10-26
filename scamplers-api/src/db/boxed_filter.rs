use diesel::{BoxableExpression, pg::Pg, prelude::*, sql_types::Bool};

type BoxedFilter<'a, QS> = Box<dyn BoxableExpression<QS, Pg, SqlType = Bool> + 'a>;
pub type OptionalBoxedFilter<'a, QS> = Option<BoxedFilter<'a, QS>>;

pub trait BoxedFilterExt<'a, QS> {
    fn new() -> Self;

    fn from_filter<O>(query: O) -> Self
    where
        O: BoxableExpression<QS, Pg, SqlType = Bool> + 'a;

    fn and_filter<O>(self, other: O) -> Self
    where
        O: BoxableExpression<QS, Pg, SqlType = Bool> + 'a;
}

impl<'a, QS: 'a> BoxedFilterExt<'a, QS> for OptionalBoxedFilter<'a, QS> {
    fn new() -> Self {
        None
    }

    fn from_filter<O>(query: O) -> Self
    where
        O: BoxableExpression<QS, Pg, SqlType = Bool> + 'a,
    {
        Some(Box::new(query))
    }

    fn and_filter<O>(self, other: O) -> Self
    where
        O: BoxableExpression<QS, Pg, SqlType = Bool> + 'a,
    {
        let Some(q) = self else {
            return Self::from_filter(other);
        };
        let other: BoxedFilter<QS> = Box::new(other);

        Some(Box::new(q.and(other)))
    }
}

pub trait AsBoxedFilter<'a, QS> {
    fn as_boxed_filter(&'a self) -> OptionalBoxedFilter<'a, QS>;
}
