use garde::Validate;
use serde::Deserialize;

use crate::db::{models::institution::NewInstitution, seed_data::index_set::IndexSetFileUrl};
mod index_set;

#[derive(Clone, Deserialize, Validate)]
pub struct SeedData {
    #[garde(dive)]
    institution: NewInstitution,
    #[garde(dive)]
    app_admin: NewPerson,
    #[garde(dive)]
    index_set_urls: Vec<IndexSetFileUrl>,
    #[garde(skip)]
    chemistries: Vec<Chemistry>,
    #[garde(dive)]
    multiplexing_tags: Vec<NewMultiplexingTag>,
    #[garde(dive)]
    library_type_specifications: Vec<NewLibraryTypeSpecification>,
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
