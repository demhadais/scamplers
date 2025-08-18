use std::sync::Arc;

use anyhow::{Context, anyhow};
use deadpool_diesel::{
    Runtime,
    postgres::{Manager as PoolManager, Pool},
};
use diesel::{PgConnection, prelude::*};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use uuid::Uuid;

use crate::{
    config::Config, db::seed_data::insert_seed_data, dev_container::DevContainer,
    result::ScamplersResult,
};

#[derive(Clone)]
pub struct AppStateCore {
    db_pool: Pool,
    http_client: reqwest::Client,
    config: Arc<Config>,
}

impl AppStateCore {
    fn new(db_pool: deadpool_diesel::postgres::Pool, config: Config) -> Self {
        Self {
            db_pool,
            http_client: reqwest::Client::new(),
            config: Arc::new(config),
        }
    }

    async fn db_conn(&self) -> ScamplersResult<deadpool_diesel::postgres::Connection> {
        Ok(self.db_pool.get().await?)
    }

    pub fn frontend_token(&self) -> &str {
        self.config.frontend_token()
    }
}

#[derive(Clone)]
struct InitialAppStateCore {
    core: AppStateCore,
    db_root_pool: Pool,
}
impl InitialAppStateCore {
    async fn db_conn(&self) -> ScamplersResult<deadpool_diesel::postgres::Connection> {
        self.core.db_conn().await
    }
}

#[derive(Clone)]
pub struct DevAppState {
    _pg_container: Arc<DevContainer>,
    core: InitialAppStateCore,
    user_id: Uuid,
}
impl DevAppState {
    async fn db_conn(&self) -> ScamplersResult<deadpool_diesel::postgres::Connection> {
        self.core.db_conn().await
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}

enum InitialAppState {
    Dev(DevAppState),
    Prod(InitialAppStateCore),
}

fn create_dev_superuser(db_conn: &mut PgConnection) -> anyhow::Result<Uuid> {
    let user_id = Uuid::now_v7();

    diesel::sql_query(format!(r#"create user "{user_id}" with superuser"#))
        .execute(db_conn)
        .context("failed to create dev superuser")?;

    Ok(user_id)
}

pub fn create_db_pool(db_url: &str) -> anyhow::Result<Pool> {
    let manager = PoolManager::new(db_url, Runtime::Tokio1);
    Ok(Pool::builder(manager).build()?)
}

impl InitialAppState {
    async fn new(config: Config) -> anyhow::Result<Self> {
        if config.is_dev() {
            let pg_container = DevContainer::new("scamplers-dev", false)
                .await
                .context("failed to start postgres container instance")?;

            let db_root_url = pg_container.db_url().await?;

            let db_root_pool = create_db_pool(&db_root_url)?;

            let user_id = db_root_pool
                .get()
                .await?
                .interact(create_dev_superuser)
                .await
                .unwrap()?;

            let db_pool = create_db_pool(&db_root_url)?;

            let core = AppStateCore::new(db_pool, config);

            Ok(Self::Dev(DevAppState {
                _pg_container: Arc::new(pg_container),
                core: InitialAppStateCore { core, db_root_pool },
                user_id,
            }))
        } else {
            let db_pool = create_db_pool(&config.db_login_url())?;
            let db_root_pool = create_db_pool(&config.db_root_url())?;

            let core = AppStateCore::new(db_pool, config);

            Ok(Self::Prod(InitialAppStateCore { core, db_root_pool }))
        }
    }

    async fn db_root_conn(&self) -> ScamplersResult<deadpool_diesel::postgres::Connection> {
        match self {
            Self::Dev(s) => s.db_conn().await,
            Self::Prod(s) => Ok(s.db_root_pool.get().await?),
        }
    }

    fn http_client(&self) -> reqwest::Client {
        match self {
            Self::Dev(s) => s.core.core.http_client.clone(),
            Self::Prod(s) => s.core.http_client.clone(),
        }
    }

    fn login_user_password(&self) -> String {
        match self {
            Self::Dev(_) => Uuid::now_v7().to_string(),
            Self::Prod(state) => state.core.config.db_login_user_password().to_string(),
        }
    }

    async fn set_login_user_password(&self) -> anyhow::Result<()> {
        const LOGIN_USER: &str = "login_user";

        let password = self.login_user_password();

        let db_conn = self.db_root_conn().await?;
        db_conn
            .interact(move |db_conn| {
                diesel::sql_query(format!(
                    r#"alter user "{LOGIN_USER}" with password '{password}'"#
                ))
                .execute(db_conn)
            })
            .await
            .unwrap()?;

        Ok(())
    }

    async fn write_seed_data(&self) -> anyhow::Result<()> {
        let seed_data = match self {
            Self::Dev(s) => s.core.core.config.seed_data()?,
            Self::Prod(s) => s.core.config.seed_data()?,
        };

        let db_conn = self.db_root_conn().await?;
        let http_client = self.http_client();
        dbg!("reachine here");

        // Thanks AI! Just have to block on the async future
        db_conn
            .interact(|db_conn: &mut PgConnection| {
                tokio::runtime::Handle::current().block_on(insert_seed_data(
                    seed_data,
                    http_client,
                    db_conn,
                ))
            })
            .await
            .unwrap()?;

        dbg!("not rech here");

        Ok(())
    }

    /// # Panics
    /// # Errors
    async fn run_migrations(&self) -> anyhow::Result<()> {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../db/migrations");

        let db_conn = self.db_root_conn().await.map_err(|e| anyhow!("{e}"))?;

        db_conn
            .interact(move |db_conn| {
                db_conn.run_pending_migrations(MIGRATIONS).unwrap();
            })
            .await
            .unwrap();

        Ok(())
    }
}

#[derive(Clone)]
pub enum AppState {
    Dev(DevAppState),
    Prod(AppStateCore),
}

impl AppState {
    pub async fn initialize(config: Config) -> anyhow::Result<Self> {
        let app_state = InitialAppState::new(config).await?;

        app_state.run_migrations().await?;
        tracing::info!("ran database migrations");

        app_state.set_login_user_password().await?;

        app_state.write_seed_data().await?;
        tracing::info!("inserted seed data");

        let app_state = match app_state {
            InitialAppState::Dev(s) => Self::Dev(s),
            InitialAppState::Prod(InitialAppStateCore { core, .. }) => Self::Prod(core),
        };

        Ok(app_state)
    }

    pub async fn db_conn(&self) -> ScamplersResult<deadpool_diesel::postgres::Connection> {
        match self {
            Self::Dev(s) => s.db_conn().await,
            Self::Prod(s) => s.db_conn().await,
        }
    }
}
