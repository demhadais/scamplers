use anyhow::{anyhow, ensure};
use diesel::PgConnection;
use scamplers_models::{
    institution,
    person::{self, Person, UserRole},
};

use crate::{
    db::{self, Operation},
    validate::Validate,
};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct InitialData {
    pub institution: institution::Creation,
    pub app_admin: person::Creation,
    // index_set_urls: Vec<Url>,
    // tenx_assays: Vec<NewTenxAssay>,
    // multiplexing_tags: Vec<NewMultiplexingTag>,
}

pub async fn insert_initial_data(
    initial_data: InitialData,
    _http_client: reqwest::Client,
    db_pool: deadpool_diesel::postgres::Pool,
) -> anyhow::Result<()> {
    fn duplicate_resource_ok<T>(result: Result<T, db::Error>) -> Result<(), db::Error> {
        let Err(err) = result else {
            return Ok(());
        };

        match err {
            db::Error::DuplicateResource { .. } => Ok(()),
            _ => Err(err),
        }
    }

    let db_conn = db_pool.get().await?;
    let mut db_conn = db_conn
        .lock()
        .map_err(|_| anyhow!("failed to lock db connection"))?;
    initial_data.validate(&mut db_conn)?;

    let InitialData {
        institution,
        mut app_admin,
        // index_set_urls,
        // tenx_assays,
        // multiplexing_tags,
    } = initial_data;

    let simple_operations = |db_conn: &mut PgConnection| -> Result<(), anyhow::Error> {
        duplicate_resource_ok(institution.execute(db_conn))?;
        //
        ensure!(
            app_admin.inner.microsoft_entra_oid.is_some(),
            "app admin must have `microsoft_entra_oid`"
        );

        app_admin.roles.push(UserRole::AppAdmin);
        let result: Result<Person, db::Error> = app_admin.execute(db_conn);
        duplicate_resource_ok(result)?;

        // // This is a loop of like 25 max
        // for assay in tenx_assays {
        //     duplicate_resource_ok(assay.execute(db_conn))?;
        // }

        // multiplexing_tags.execute(db_conn)?;

        Ok(())
    };

    // Insert index sets first so we can insert library-type specifications as part
    // of a larger set of operations in one shot
    // let db_conn = db_pool.get().await?;
    // download_and_insert_index_sets(&index_set_urls, http_client, db_conn).await?;

    let db_conn = db_pool.get().await?;
    db_conn.interact(simple_operations).await.unwrap()?;

    Ok(())
}
