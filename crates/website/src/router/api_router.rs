use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use database::models::{Gardyn, Plant};

use crate::state::ApplicationState;
use crate::states::pool_wrapper::PoolWrapper;

pub fn build_api_router(state: ApplicationState) -> Router {
    Router::new()
        .route("/gardyns", get(gardyns))
        .route("/gardyn/{id}/plants", get(plants))
        .with_state(state)
}

async fn gardyns(State(PoolWrapper(pool)): State<PoolWrapper>) -> Json<Vec<Gardyn>> {
    let mut connection = pool.get().await.unwrap();

    let gardyns = database::gardyns::get_all(&mut connection).await.unwrap();

    Json(gardyns)
}

async fn plants(
    Path(id): Path<i32>,
    State(PoolWrapper(pool)): State<PoolWrapper>,
) -> Json<Vec<Plant>> {
    let mut connection = pool.get().await.unwrap();

    let plants = database::plants::get_by_gardyn_id(&mut connection, id)
        .await
        .unwrap();

    Json(plants)
}
