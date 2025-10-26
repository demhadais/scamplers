pub trait AsIlike {
    fn as_ilike(&self) -> String;
}

impl AsIlike for str {
    fn as_ilike(&self) -> String {
        format!("%{self}%")
    }
}

impl AsIlike for String {
    fn as_ilike(&self) -> String {
        self.as_str().as_ilike()
    }
}

impl AsIlike for &[String] {
    fn as_ilike(&self) -> String {
        self.join("|").as_ilike()
    }
}

impl AsIlike for &Vec<String> {
    fn as_ilike(&self) -> String {
        self.as_slice().as_ilike()
    }
}

#[macro_export]
macro_rules! init_stmt {
    ($select:ident, query = $query:expr, orderby_spec = {$($enum_variant:path => $db_col:expr),*}) => {
        {
            let mut stmt = $select::query()
            .limit($query.limit)
            .offset($query.offset)
            .into_boxed();

            for ordering in &$query.order_by {
                stmt = match (ordering.field, ordering.descending) {
                    $(
                        ($enum_variant, true) => stmt.then_order_by($db_col.desc()),
                        ($enum_variant, _) => stmt.then_order_by($db_col.asc()),
                    )*
                }
            }
            stmt
        }
    };
}

#[macro_export]
macro_rules! apply_eq_any_filters {
    ($stmt:expr, filters = {$($col:expr => $values:expr),*}) => {{
        $(
            if !$values.is_empty() {
                $stmt = $stmt.filter($col.eq_any($values));
            }
        )*

        $stmt
    }};
}

#[macro_export]
macro_rules! apply_eq_filters {
    ($stmt:expr, filters = {$($col:expr => $value:expr),*}) => {{
        $(
            if let Some(value) = $value {
                $stmt = $stmt.filter($col.eq(value));
            }
        )*

        $stmt
    }};
}

#[macro_export]
macro_rules! apply_time_filters {
    ($stmt:expr, filters = {$($col:expr => ($before:expr, $after:expr)),*}) => {{
        $(
            if let Some(before) = $before {
                $stmt = $stmt.filter($col.lt(before));
            }

            if let Some(after) = $after {
                $stmt = $stmt.filter($col.gt(after));
            }
        )*

        $stmt
    }};
}

#[macro_export]
macro_rules! apply_ilike_filters {
    ($stmt:expr, filters = {$($col:expr => $strings:expr),*}) => {{
        use $crate::db::utils::AsIlike;

        $(
            if !$strings.is_empty() {
                $stmt = $stmt.filter($col.ilike($strings.as_ilike()))
            }
        )*

        $stmt
    }};
}

#[macro_export]
macro_rules! apply_jsonb_contains_filters {
    ($stmt:expr, filters = {$($col:expr => $value:expr),*}) => {{
        $(
            let mut jsonb_condition: Option<Box<dyn diesel::BoxableExpression<_, diesel::pg::Pg, SqlType = diesel::sql_types::Nullable<diesel::sql_types::Bool>>>> = None;

            for jsonb in $value {
                if let Some(condition) = jsonb_condition {
                    jsonb_condition = Some(Box::new(condition.or($col.contains(jsonb))));
                } else {
                    jsonb_condition = Some(Box::new($col.contains(jsonb)));
                }
            }

            if let Some(condition) = jsonb_condition {
                $stmt = $stmt.filter(condition);
            }
        )*

        $stmt
    }};
}

#[macro_export]
macro_rules! impl_id_db_operation {
    (
        id_type =
        $type_:path,delegate_to =
        $delegated:ident,returns =
        $ret:path,resource =
        $resource:expr
    ) => {
        impl $crate::db::Operation<$ret> for $type_ {
            fn execute(
                self,
                db_conn: &mut diesel::PgConnection,
            ) -> Result<$ret, $crate::db::Error> {
                let query = $delegated::builder().ids(self).build();
                let results = query.execute(db_conn)?;

                return Ok(results.into_iter().next().ok_or(
                    $crate::db::error::Error::ResourceNotFound {
                        resource: $resource.to_string(),
                        resource_id: self.into(),
                    },
                )?);
            }
        }
    };
}
