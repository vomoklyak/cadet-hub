use sqlx::query_builder::Separated;
use sqlx::{Encode, QueryBuilder, Sqlite, Type};

pub(super) enum InsertMode {
    OnConflictFail,
    OnConflictSkip,
}

pub(super) fn add_in_clause<'q, T>(
    query_builder: &mut QueryBuilder<'q, Sqlite>,
    column_name: &str,
    values_opt: &'q Option<Vec<T>>,
) where
    T: 'q + Type<Sqlite> + Encode<'q, Sqlite> + Send + Sync,
{
    if let Some(values) = values_opt {
        if !values.is_empty() {
            query_builder.push(format!(" AND {} IN (", column_name));
            let mut separated: Separated<Sqlite, &str> = query_builder.separated(", ");
            for value in values {
                separated.push_bind(value);
            }
            separated.push_unseparated(")");
        }
    }
}

pub(super) fn add_pagination(
    query_builder: &mut QueryBuilder<Sqlite>,
    order_by_columns: &str,
    limit: i64,
    offset: i64,
) {
    query_builder.push(format!(" ORDER BY {order_by_columns} "));
    query_builder.push(" LIMIT ");
    query_builder.push_bind(limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);
}
