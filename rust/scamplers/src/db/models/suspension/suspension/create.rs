use scamplers_macros::db_insertion;
use uuid::Uuid;

#[db_insertion]
#[diesel(table_name = scamplers_schema::suspension_preparers)]
struct SuspensionPreparer {
    suspension_id: Uuid,
    prepared_by: Uuid,
}
