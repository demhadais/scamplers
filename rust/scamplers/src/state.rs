use std::sync::Arc;

use diesel::PgConnection;
use futures::lock::Mutex;

use crate::{
    auth::User,
    db::{DbOperation, models::institution::NewInstitution},
    dev_container::DevContainer,
};

#[derive(Clone)]
enum AppState {
    Dev {
        db_pool: deadpool_diesel::postgres::Pool,
        _pg_container: Arc<DevContainer>,
        user_id: User,
        http_client: reqwest::Client,
        config: Arc<Config>,
    },
    Prod {
        db_pool: Pool<AsyncPgConnection>,
        db_root_pool: Option<Pool<AsyncPgConnection>>,
        http_client: reqwest::Client,
        config: Arc<Config>,
    },
}
impl AppState {
    async fn new(config: Config) -> anyhow::Result<Self> {
        let manager = deadpool_diesel::postgres::Manager::new(
            "database_url",
            deadpool_diesel::Runtime::Tokio1,
        );
        let pool = deadpool_diesel::postgres::Pool::builder(manager)
            .build()
            .unwrap();
        let mut x = pool.get().await.unwrap();
        let y = x
            .interact(|conn| NewInstitution {}.execute_as_user("user", conn).unwrap())
            .await
            .unwrap();
        NewInstitution::execute_as_user(self, "user", &mut x).unwrap();
        let container_err = "failed to start postgres container instance";

        let state = if config.is_dev() {
            let pg_container = DevContainer::new("scamplers-dev", false)
                .await
                .context(container_err)?;
            let db_root_url = pg_container.db_url().await?;

            let mut db_conn = AsyncPgConnection::establish(&db_root_url).await?;
            let user_id = Uuid::now_v7();
            diesel::sql_query(format!(r#"create user "{user_id}" with superuser"#))
                .execute(&mut db_conn)
                .await
                .context("failed to create dev superuser")?;

            let db_config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&db_root_url);
            let db_pool = Pool::builder(db_config).build()?;

            Self::Dev {
                db_pool,
                _pg_container: Arc::new(pg_container),
                user_id,
                http_client: reqwest::Client::new(),
                config: Arc::new(config),
            }
        } else {
            let db_config =
                AsyncDieselConnectionManager::<AsyncPgConnection>::new(config.db_login_url());
            let db_pool = Pool::builder(db_config).build()?;

            let db_root_config =
                AsyncDieselConnectionManager::<AsyncPgConnection>::new(config.db_root_url());
            let db_root_pool = Some(Pool::builder(db_root_config).max_size(1).build()?);

            Self::Prod {
                db_pool,
                db_root_pool,
                http_client: reqwest::Client::new(),
                config: Arc::new(config),
            }
        };

        Ok(state)
    }

    pub async fn db_conn(
        &self,
    ) -> db::error::Result<diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection>>
    {
        use AppState::{Dev, Prod};

        match self {
            Dev { db_pool, .. } | Prod { db_pool, .. } => Ok(db_pool.get().await?),
        }
    }

    async fn db_root_conn(
        &self,
    ) -> db::error::Result<diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection>>
    {
        use AppState::Prod;

        let Prod { db_root_pool, .. } = self else {
            return self.db_conn().await;
        };

        let Some(db_root_pool) = db_root_pool else {
            return Err(db::error::Error::Other {
                message: "root user connection to database should not be required at this stage"
                    .to_string(),
            });
        };

        Ok(db_root_pool.get().await?)
    }

    // In theory, this should be two separate functions - one that actually does the password setting, and one that
    // constructs the arguments. This is the only time this sequence of events happens, so we can keep it as is.
    // Also, this shouldn't be a method of `AppState`
    async fn set_login_user_password(&self) -> anyhow::Result<()> {
        const LOGIN_USER: &str = "login_user";

        let password = match self {
            AppState::Dev { .. } => Uuid::now_v7().to_string(),
            AppState::Prod { config, .. } => config.db_login_user_password().to_string(),
        };

        let mut db_conn = self.db_root_conn().await?;
        diesel::sql_query(format!(
            r#"alter user "{LOGIN_USER}" with password '{password}'"#
        ))
        .execute(&mut db_conn)
        .await?;

        Ok(())
    }

    // TODO: This also shouldn't be a method of `AppState`
    async fn write_seed_data(&self) -> anyhow::Result<()> {
        use AppState::{Dev, Prod};

        let mut db_conn = self.db_root_conn().await?;

        match self {
            Dev {
                http_client,
                config,
                ..
            }
            | Prod {
                http_client,
                config,
                ..
            } => {
                let seed_data = config.seed_data()?;
                seed_data.write(&mut db_conn, http_client.clone()).await
            }
        }
    }

    fn drop_db_root_pool(&mut self) {
        use AppState::{Dev, Prod};

        match self {
            Dev { .. } => (),
            Prod { db_root_pool, .. } => {
                *db_root_pool = None;
            }
        }
    }
}
