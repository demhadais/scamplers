use anyhow::{Context, anyhow};
use deadpool_diesel::{
    Runtime,
    postgres::{Manager as PoolManager, Pool},
};
use diesel::{PgConnection, prelude::*};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use uuid::Uuid;

use crate::{config::Config, db, initial_data::insert_initial_data};

#[derive(Clone)]
pub enum AppState {
    Dev {
        db_pool: Pool,
        user_id: Uuid,
    },
    Prod {
        db_pool: Pool,
        frontend_token: String,
    },
}

fn create_db_pool(db_url: &str) -> anyhow::Result<Pool> {
    let manager = PoolManager::new(db_url, Runtime::Tokio1);
    Ok(Pool::builder(manager).build()?)
}

fn create_dev_superuser(db_conn: &mut PgConnection) -> anyhow::Result<Uuid> {
    let user_id = Uuid::now_v7();

    diesel::sql_query(format!(r#"create user "{user_id}" with superuser"#))
        .execute(db_conn)
        .context("failed to create dev superuser")?;

    Ok(user_id)
}

fn run_migrations(db_conn: &mut PgConnection) -> anyhow::Result<()> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../scamplers-schema/migrations");

    db_conn
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow!(e))?;

    Ok(())
}

fn set_login_user_password(password: &str, db_conn: &mut PgConnection) -> anyhow::Result<()> {
    const LOGIN_USER: &str = "login_user";

    diesel::sql_query(format!(
        r#"alter user "{LOGIN_USER}" with password '{password}'"#
    ))
    .execute(db_conn)?;

    Ok(())
}

impl AppState {
    pub async fn initialize(config: &mut Config) -> anyhow::Result<Self> {
        let mut root_db_conn = PgConnection::establish(&config.db_root_url())
            .context("failed to connect to db as root to run migrations")?;

        run_migrations(&mut root_db_conn)?;
        tracing::info!("ran database migrations");

        set_login_user_password(&config.db_login_user_password(), &mut root_db_conn)?;
        tracing::info!("set password for db login user");

        let db_url = if config.dev() {
            config.db_root_url()
        } else {
            config.db_login_url()
        };

        let db_pool = create_db_pool(&db_url)?;

        let initial_data = config
            .initial_data()
            .context("failed to read initial data")?;
        insert_initial_data(initial_data, reqwest::Client::new(), db_pool.clone())
            .await
            .context("failed to insert initial data")?;
        tracing::info!("inserted initial data");

        let db_url = if config.dev() {
            config.db_root_url()
        } else {
            config.db_login_url()
        };

        let db_pool = create_db_pool(&db_url)?;

        let state = if config.dev() {
            let mut db_conn = PgConnection::establish(&config.db_root_url())?;
            let user_id = create_dev_superuser(&mut db_conn)?;
            Self::Dev { db_pool, user_id }
        } else {
            Self::Prod {
                db_pool,
                frontend_token: config.frontend_token().to_string(),
            }
        };

        Ok(state)
    }

    pub async fn db_conn(&self) -> Result<deadpool_diesel::postgres::Connection, db::Error> {
        match self {
            Self::Dev { db_pool, .. } | Self::Prod { db_pool, .. } => Ok(db_pool.get().await?),
        }
    }
}
