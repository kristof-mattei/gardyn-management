use std::sync::Arc;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;

use crate::states::config::Config;
use crate::states::pool_wrapper::PoolWrapper;

/// This is to be able to do
/// ```no_run
/// async fn get_handler(State(config): State<Arc<Config>>) -> impl IntoResponse {
///     // ...
/// }
/// ```
///
/// Note that `Arc::<Config>` then is cloned.
impl FromRef<ApplicationState> for Arc<Config> {
    fn from_ref(state: &ApplicationState) -> Self {
        Arc::clone(&state.config)
    }
}

/// This is to be able to do
/// ```no_run
/// async fn get_handler(State(config): State<PoolWrapper>) -> impl IntoResponse {
///     // ...
/// }
/// ```
///
/// Note that `PoolWrapper` then is cloned.
impl FromRef<ApplicationState> for PoolWrapper {
    fn from_ref(state: &ApplicationState) -> Self {
        state.pool.clone()
    }
}

#[derive(Clone)]
pub struct ApplicationState {
    pub config: Arc<Config>,
    pub pool: PoolWrapper,
}

impl ApplicationState {
    #[must_use]
    pub fn new(config: Config, pool: Pool<AsyncPgConnection>) -> Self {
        ApplicationState {
            config: Arc::new(config),
            pool: PoolWrapper(pool),
        }
    }
}

/// This is to be able to do
/// ```no_run
/// async fn get_handler(state: ApplicationState) -> impl IntoResponse {
///     // ...
/// }
/// ```
///
/// Note that `ApplicationState` then is cloned.
impl<S> FromRequestParts<S> for ApplicationState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    // TODO State not found error
    type Rejection = ();

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
