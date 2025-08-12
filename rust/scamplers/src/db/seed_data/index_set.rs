use anyhow::Context;
use diesel::PgConnection;
use garde::Validate;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::db::{DbOperation, models::index_set::NewIndexSets};

#[derive(Deserialize, Validate, Clone, Serialize, Debug)]
#[serde(transparent)]
pub(super) struct IndexSetFileUrl(#[garde(custom(is_10x_genomics_url))] Url);

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_10x_genomics_url(url: &Url, _: &()) -> garde::Result {
    let Some(domain) = url.domain() else {
        return Err(garde::Error::new("malformed URL"));
    };

    if domain != "cdn.10xgenomics.com" {
        return Err(garde::Error::new(format!(
            "URL domain must be 'cdn.10xgenomics.com', found {domain}"
        )));
    }

    Ok(())
}

impl IndexSetFileUrl {
    pub(super) async fn download(
        self,
        http_client: reqwest::Client,
    ) -> anyhow::Result<NewIndexSets> {
        Ok(http_client.get(self.0.clone()).send().await?.json().await?)
    }
}

// We use anyhow::Result here because we just want to know what went wrong, we
// don't care about serializing structured errors to a client
pub(super) async fn download_and_insert_index_sets(
    db_conn: &mut PgConnection,
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
        sets.validate()?;
        match sets {
            NewIndexSets::Dual(s) => s.execute(db_conn)?,
            NewIndexSets::Single(s) => s.execute(db_conn)?,
        }
    }

    Ok(())
}
