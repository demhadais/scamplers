use diesel::PgConnection;
use garde::Validate;
use scamplers_macros::base_model;
use url::Url;

use crate::{
    db::{
        DbOperation,
        models::{
            chemistry::Chemistry,
            institution::NewInstitution,
            library_type_specification::LibraryTypeSpecification,
            multiplexing_tag::NewMultiplexingTag,
            person::{NewPerson, Person, UserRole},
        },
        seed_data::index_set::{download_and_insert_index_sets, is_10x_genomics_url},
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
    chemistries: Vec<Chemistry>,
    #[garde(dive)]
    multiplexing_tags: Vec<NewMultiplexingTag>,
    #[garde(dive)]
    library_type_specifications: Vec<LibraryTypeSpecification>,
}

pub async fn insert_seed_data(
    seed_data: SeedData,
    http_client: reqwest::Client,
    db_conn: &mut PgConnection,
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
        chemistries,
        multiplexing_tags,
        library_type_specifications,
    } = seed_data;

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

    download_and_insert_index_sets(&index_set_urls, http_client, db_conn).await?;

    chemistries.execute(db_conn)?;

    multiplexing_tags.execute(db_conn)?;

    library_type_specifications.execute(db_conn)?;

    Ok(())
}
