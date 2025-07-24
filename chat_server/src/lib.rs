mod config;
mod error;
mod handlers;
mod middlewares;
mod models;
mod utils;
use anyhow::Context;
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};
use sqlx::PgPool;
#[cfg(test)]
use sqlx_db_tester::TestPg;
use std::{fmt, ops::Deref, sync::Arc};

pub use config::AppConfig;
pub use error::{AppError, ErrorOutput};
use handlers::*;
pub use models::*;
use utils::{DecodingKey, EncodingKey};

use crate::middlewares::{set_layer, verify_token};
#[derive(Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) dk: DecodingKey,
    pub(crate) ek: EncodingKey,
    pub(crate) pool: PgPool,
}

pub async fn get_router(config: AppConfig) -> Result<Router, AppError> {
    let state = AppState::try_new(config).await?;
    let api = Router::new()
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/{id}",
            patch(update_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route("/chat/{id}/messages", get(list_message_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler));

    let app = Router::new()
        .route("/", axum::routing::get(index_handler))
        .nest("/api", api)
        .with_state(state);
    Ok(set_layer(app))
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;
        let pool = PgPool::connect(&config.server.db_url)
            .await
            .context("Failed to connect to the database")?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
            }),
        })
    }
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(test)]
impl AppState {
    pub async fn new_for_test(config: AppConfig) -> Result<(TestPg, Self), AppError> {
        use sqlx_db_tester::TestPg;

        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load sk failed")?;
        let tdb = TestPg::new(
            "postgres://chat:Pyh20010226@localhost:5432".to_string(),
            std::path::Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let state = Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
            }),
        };
        Ok((tdb, state))
    }
}
