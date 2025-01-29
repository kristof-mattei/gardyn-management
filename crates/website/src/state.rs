use std::sync::Arc;

use axum::extract::FromRef;

use crate::states::config::Config;
use crate::states::pool_wrapper::PoolWrapper;

impl FromRef<ApplicationState> for Arc<Config> {
    fn from_ref(input: &ApplicationState) -> Self {
        input.config.clone()
    }
}

impl FromRef<ApplicationState> for PoolWrapper {
    fn from_ref(input: &ApplicationState) -> Self {
        input.pool.clone()
    }
}

#[derive(Clone)]
pub struct ApplicationState {
    pub config: Arc<Config>,
    pub pool: PoolWrapper,
}

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;

impl ApplicationState {
    #[must_use]
    pub fn new(config: Config, pool: Pool<AsyncPgConnection>) -> Self {
        ApplicationState {
            config: Arc::new(config),
            pool: PoolWrapper(pool),
        }
    }
}

// impl<S> FromRequestParts<S> for ApplicationState
// where
//     Self: FromRef<S>,
//     S: Send + Sync,
// {
//     // TODO State not found error
//     type Rejection = ();

//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         Ok(Self::from_ref(state))
//     }
// }
