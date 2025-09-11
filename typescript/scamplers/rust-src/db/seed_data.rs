use diesel::PgConnection;
use garde::Validate;
use scamplers_macros::base_model;
use url::Url;

use crate::{
    db::{
        DbOperation,
        models::{
            institution::NewInstitution,
            multiplexing_tag::NewMultiplexingTag,
            person::{NewPerson, Person, UserRole},
            tenx_assay::NewTenxAssay,
        },
        seed_data::index_set::download_and_insert_index_sets,
        tenx_url_validator::is_10x_genomics_url,
    },
    result::{ScamplersError, ScamplersResult, ServerError},
};
mod index_set;

#[base_model]
pub struct SeedData {
    #[garde(dive)]
    institution: NewInstitution,
    #[garde(dive)]
    app_admin: NewPerson,
    #[valuable(skip)]
    #[garde(inner(custom(is_10x_genomics_url)))]
    index_set_urls: Vec<Url>,
    #[garde(dive)]
    tenx_assays: Vec<NewTenxAssay>,
    #[garde(dive)]
    multiplexing_tags: Vec<NewMultiplexingTag>,
}

/// # Errors
pub async fn insert_seed_data(
    seed_data: SeedData,
    http_client: reqwest::Client,
    db_pool: deadpool_diesel::postgres::Pool,
) -> anyhow::Result<()> {
    fn duplicate_resource_ok<T>(result: ScamplersResult<T>) -> ScamplersResult<()> {
        if matches!(result, Err(ScamplersError::DuplicateResource(_))) {
        } else {
            result?;
        }

        Ok(())
    }

    seed_data.validate().map_err(|e| ServerError {
        message: e.to_string(),
        ..Default::default()
    })?;

    let SeedData {
        institution,
        mut app_admin,
        index_set_urls,
        tenx_assays,
        multiplexing_tags,
    } = seed_data;

    let simple_operations = |db_conn: &mut PgConnection| -> ScamplersResult<()> {
        duplicate_resource_ok(institution.execute(db_conn))?;

        if app_admin.ms_user_id.is_none() {
            return Err(ServerError {
                message: "app admin must have ms_user_id".to_string(),
                ..Default::default()
            })?;
        }

        app_admin.roles.push(UserRole::AppAdmin);
        let result: ScamplersResult<Person> = app_admin.execute(db_conn);
        duplicate_resource_ok(result)?;

        // This is a loop of like 25 max
        for assay in tenx_assays {
            duplicate_resource_ok(assay.execute(db_conn))?;
        }

        multiplexing_tags.execute(db_conn)?;

        Ok(())
    };

    // Insert index sets first so we can insert library-type specifications as part
    // of a larger set of operations in one shot
    let db_conn = db_pool.get().await?;
    download_and_insert_index_sets(&index_set_urls, http_client, db_conn).await?;

    let db_conn = db_pool.get().await?;
    db_conn.interact(simple_operations).await.unwrap()?;

    Ok(())
}
