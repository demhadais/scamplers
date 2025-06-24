use crate::db::seed_data::library_type_specification::LibraryTypeSpecification;

use super::model::Write;
use admin::NewAdmin;
use anyhow::Context;
use diesel_async::AsyncPgConnection;
use garde::Validate;
use index_set::IndexSetFileUrl;
use scamplers_core::model::{chemistry::Chemistry, institution::NewInstitution};
use serde::Deserialize;
mod admin;
mod chemistry;
mod index_set;
mod library_type_specification;

#[derive(Deserialize, Clone, Validate)]
pub struct SeedData {
    #[garde(dive)]
    institution: NewInstitution,
    #[garde(dive)]
    app_admin: NewAdmin,
    #[garde(dive)]
    index_set_urls: Vec<IndexSetFileUrl>,
    #[garde(skip)]
    chemistries: Vec<Chemistry>,
    #[garde(skip)]
    library_type_specification: LibraryTypeSpecification,
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
            library_type_specification,
        } = self;

        let institutions_result = institution.write(db_conn).await;
        if !matches!(
            institutions_result,
            Err(super::error::Error::DuplicateRecord { .. }) | Ok(_)
        ) {
            institutions_result?;
        }

        app_admin.write(db_conn).await?;
        download_and_insert_index_sets(db_conn, http_client, &index_set_urls).await?;
        chemistries.write(db_conn).await?;

        Ok(())
    }
}

// We use anyhow::Result here because we just want to know what went wrong, we
// don't care about serializing structured errors to a client
async fn download_and_insert_index_sets(
    db_conn: &mut AsyncPgConnection,
    http_client: reqwest::Client,
    file_urls: &[IndexSetFileUrl],
) -> anyhow::Result<()> {
    let downloads = file_urls
        .iter()
        .map(|url| url.clone().download(http_client.clone()));
    let index_sets = futures::future::try_join_all(downloads)
        .await
        .context("failed to download index set files")?;

    // A for-loop is fine because this is like 10 URLs max, and each of these is a
    // bulk insert
    for sets in index_sets {
        sets.write(db_conn)
            .await
            .context("failed to insert index sets into database")?;
    }

    Ok(())
}
