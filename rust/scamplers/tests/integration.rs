#![cfg(feature = "app")]

use axum::http::StatusCode;
use pretty_assertions::assert_eq;
use rand::seq::IteratorRandom;
use rstest::rstest;
use scamplers::{
    client::ScamplersClient,
    config::Config,
    db::models::institution::{InstitutionQuery, NewInstitution},
    dev_container::DevContainer,
    result::ScamplersErrorResponse,
    server::{self},
};
use serde_json::json;
use tokio::runtime::Runtime;
use uuid::Uuid;

struct TestState {
    _container: Option<DevContainer>,
    config: Config,
    server_handle: tokio::task::JoinHandle<anyhow::Result<()>>,
}

impl TestState {
    async fn new(dev_api: bool) -> Self {
        let (container, db_root_password, db_login_user_password, db_host, db_port) = if dev_api {
            (
                None,
                String::new(),
                String::new(),
                String::default(),
                u16::default(),
            )
        } else {
            let container = DevContainer::new(&Uuid::now_v7().to_string(), !dev_api)
                .await
                .unwrap();
            let db_root_password = container.password().unwrap().to_string();
            let db_login_user_password = Uuid::now_v7().to_string();
            let db_host = container.db_host().await.unwrap();
            let db_port = container.db_port().await.unwrap();

            (
                Some(container),
                db_root_password,
                db_login_user_password,
                db_host,
                db_port,
            )
        };

        let institution_id = Uuid::now_v7();

        let seed_data = json!({
              "institution": {
                "id": institution_id,
                "name": "Hogwarts School for Witchcraft and Wizardry"
              },
              "app_admin": {
                "name": "Ahmed",
                "email": "ahmed.said@jax.org",
                "institution_id": institution_id,
                "ms_user_id": Uuid::now_v7(),
              },
              "index_set_urls": [],
              "library_type_specifications": [],
              "chemistries": [],
              "multiplexing_tags": []
            }
        );

        let port = (7000..8000).choose(&mut rand::rng()).unwrap();

        let config = json!({
          "dev": dev_api,
          "db_root_user": "postgres",
          "db_root_password": db_root_password,
          "db_login_user_password": db_login_user_password,
          "db_host": db_host,
          "db_port": db_port,
          "db_name": "postgres",
          "frontend_token": "",
          "host": "localhost",
          "port": port,
          "seed_data": seed_data
        });

        let config: Config = serde_json::from_value(config).unwrap();

        let server_handle =
            tokio::runtime::Handle::current().spawn(server::serve_integration_test(config.clone()));
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        if server_handle.is_finished() {
            server_handle.await.unwrap().unwrap();
            panic!("server startup failed");
        }

        Self {
            _container: container,
            config,
            server_handle,
        }
    }
}

#[rstest]
#[tokio::test(flavor = "multi_thread")]
#[ignore = "this test does not work"]
async fn prod_api_auth() {
    use scamplers::result::PermissionDeniedError;
    let test_state = TestState::new(false).await;

    let app_address = format!("http://{}", test_state.config.app_address());

    // A client with no API key
    let client = ScamplersClient::new(app_address.clone(), None, None, false);
    let request = NewInstitution::builder()
        .id(Uuid::now_v7())
        .name("institution")
        .build();
    let expected_error = ScamplersErrorResponse::builder()
        .status(StatusCode::UNAUTHORIZED)
        .error(
            PermissionDeniedError::builder()
                .message("invalid API key")
                .build(),
        )
        .build();
    let received_error = client.send_request(request.clone()).await.unwrap_err();
    assert_eq!(expected_error, received_error);

    // A client with an invalid API key
    let client = ScamplersClient::new(
        app_address,
        None,
        Some("krabby patty secret formular".to_string()),
        false,
    );
    let received_error = client.send_request(request).await.unwrap_err();
    assert_eq!(expected_error, received_error);

    test_state.server_handle.abort();
}

#[rstest]
#[tokio::test(flavor = "multi_thread")]
#[ignore = "this test does not work"]
async fn client_deserialization() {
    let test_state = TestState::new(true).await;

    let app_address = format!("http://{}", test_state.config.app_address());

    // A client with no API key
    let client = ScamplersClient::new(app_address, None, None, false);

    let request = NewInstitution::builder()
        .id(Uuid::now_v7())
        .name("institution")
        .build();
    client.send_request(request).await.unwrap();

    let request = InstitutionQuery::default();
    let response = client.send_request(request).await.unwrap();
    assert!(response.len() == 2);

    test_state.server_handle.abort();
}
