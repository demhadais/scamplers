use anyhow::Context;
use garde::Validate;
use url::Url;

use crate::db::{DbOperation, models::index_set::NewIndexSets};

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_10x_genomics_url(url: &Url, _: &()) -> garde::Result {
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

async fn download_index_set(
    http_client: reqwest::Client,
    url: &str,
) -> anyhow::Result<NewIndexSets> {
    Ok(http_client.get(url).send().await?.json().await?)
}

// We use anyhow::Result here because we just want to know what went wrong, we
// don't care about serializing structured errors to a client
pub(super) async fn download_and_insert_index_sets(
    file_urls: &[Url],
    http_client: reqwest::Client,
    db_conn: deadpool_diesel::postgres::Connection,
) -> anyhow::Result<()> {
    let downloads = file_urls
        .iter()
        .map(|url| download_index_set(http_client.clone(), url.as_str()));

    // Do this as joined futures because I am speed
    let index_sets = futures::future::try_join_all(downloads)
        .await
        .context("failed to download index set files")?;

    // A for-loop is fine because this is like 10 URLs max, and each of these is a
    // bulk insert
    for sets in index_sets {
        sets.validate()?;
        match sets {
            NewIndexSets::Dual(s) => db_conn
                .interact(|db_conn| s.execute(db_conn))
                .await
                .unwrap()?,
            NewIndexSets::Single(s) => db_conn
                .interact(|db_conn| s.execute(db_conn))
                .await
                .unwrap()?,
        }
    }

    Ok(())
}
