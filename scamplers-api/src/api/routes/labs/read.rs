use diesel::{SelectableExpression, prelude::*};
use scamplers_models::lab;
use scamplers_schema::labs;

use crate::db::{BoxedFilter, BoxedFilterExt, ToBoxedFilter};

impl<'a, QS: 'a> ToBoxedFilter<'a, QS> for lab::Filter
where
    labs::id: SelectableExpression<QS>,
    labs::name: SelectableExpression<QS>,
{
    fn to_boxed_filter(&'a self) -> BoxedFilter<'a, QS> {
        let mut filter = BoxedFilter::new();

        if let Some(ids) = self.ids() {
            filter = filter.and_condition(labs::id.eq_any(ids));
        }

        if let Some(name) = self.name() {
            filter = filter.and_condition(labs::name.like(name));
        }

        filter
    }
}
