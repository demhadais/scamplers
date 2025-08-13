use anyhow::ensure;
use axum_extra::headers::Server;
use garde::Validate;
use scamplers_macros::base_model;
use url::Url;

use crate::{
    db::{
        DbOperation,
        models::{
            chemistry::Chemistry,
            institution::NewInstitution,
            library_type_specification::NewLibraryTypeSpecification,
            multiplexing_tag::NewMultiplexingTag,
            person::{NewPerson, UserRole},
        },
        seed_data::index_set::is_10x_genomics_url,
    },
    result::{ScamplersError, ServerError},
};
mod index_set;

#[base_model]
pub struct SeedData {
    #[garde(dive)]
    institution: NewInstitution,
    #[garde(dive)]
    app_admin: NewPerson,
    #[garde(inner(custom(is_10x_genomics_url)))]
    index_set_urls: Vec<Url>,
    #[garde(dive)]
    chemistries: Vec<Chemistry>,
    #[garde(dive)]
    multiplexing_tags: Vec<NewMultiplexingTag>,
    #[garde(dive)]
    library_type_specifications: Vec<NewLibraryTypeSpecification>,
}

impl DbOperation<()> for SeedData {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> crate::result::ScamplersResult<()> {
        self.validate().map_err(|e| ServerError {
            message: e.to_string(),
            ..Default::default()
        })?;

        let Self {
            institution,
            mut app_admin,
            index_set_urls,
            chemistries,
            multiplexing_tags,
            library_type_specifications,
        } = self;

        let institution_result = institution.execute(db_conn);
        if let Err(error) = &institution_result
            && matches!(error, ScamplersError::DuplicateResource(_))
        {
        } else {
            institution_result?;
        }

        ensure!(
            app_admin.ms_user_id.is_some(),
            ServerError {
                message: "app admin must have ms_user_id".to_string(),
                ..Default::default()
            }
        );
        app_admin.roles.push(UserRole::AppAdmin);
        app_admin
    }
}

impl SeedData {
    /// # Errors
    pub async fn write(
        self,
        db_conn: &mut AsyncPgConnection,
        http_client: reqwest::Client,
    ) -> anyhow::Result<()> {
        self.validate()?;

        let Self {
            institution,
            app_admin,
            index_set_urls,
            chemistries,
            multiplexing_tags,
            library_type_specifications,
        } = self;

        let institution_result = institution.write_to_db(db_conn).await;
        if let Err(error) = &institution_result
            && matches!(error.inner(), ScamplersCoreError::DuplicateResource(_))
        {
        } else {
            institution_result?;
        }

        app_admin.write(db_conn).await?;
        download_and_insert_index_sets(db_conn, http_client, &index_set_urls).await?;
        chemistries.write_to_db(db_conn).await?;
        multiplexing_tags.write_to_db(db_conn).await?;
        library_type_specifications.write_to_db(db_conn).await?;

        Ok(())
    }
}
