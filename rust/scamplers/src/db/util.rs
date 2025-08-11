pub trait AsIlike {
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

#[macro_export]
macro_rules! init_stmt {
    (stmt = $stmt_base:expr, query = $query:expr, output = $output_struct:ident; $($enum_variant:path => $db_col:expr)*) => {
        {
            let mut stmt = $stmt_base
            .select($output_struct::as_select())
            .limit($query.pagination.limit)
            .offset($query.pagination.offset)
            .into_boxed();

            for ordering in &$query.order_by {
                stmt = match ordering {
                    $(
                        $enum_variant { descending: true } => stmt.then_order_by($db_col.desc()),
                        $enum_variant { .. } => stmt.then_order_by($db_col.asc())
                    )*
                }
            }
            stmt
        }
    };
}
