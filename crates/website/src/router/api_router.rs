use std::time::Duration;

use axum::Router;
use axum::{extract::Json, routing::get};
use axum::{
    extract::{Path, State},
    routing::post,
};
use database::models::{Gardyn, GardynSlot, Plant};
use serde::Deserialize;

use crate::state::ApplicationState;
use crate::states::pool_wrapper::PoolWrapper;

pub fn build_api_router(state: ApplicationState) -> Router {
    Router::new()
        .route("/gardyns", get(gardyns))
        .route("/gardyn/{id}/plants", get(plants))
        .route("/plants/{plant_id}/move", post(move_plant))
        .route("/plants/{id}/pause", get(plants))
        .route("/plants/{id}/terminate", get(plants))
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
) -> Json<Vec<(GardynSlot, Option<Plant>)>> {
    let mut connection = pool.get().await.unwrap();

    let plants = database::gardyns::get_plants(&mut connection, id)
        .await
        .unwrap();

    Json(plants)
}

#[derive(Deserialize)]
struct MovePlant {
    gardyn_id: i32,
    x: i32,
    y: i32,
}

async fn move_plant(
    Path(_plant_id): Path<i32>,
    State(PoolWrapper(_pool)): State<PoolWrapper>,
    Json(_move_plant): Json<MovePlant>,
) -> Json<Vec<Plant>> {
    tokio::time::sleep(Duration::from_secs(9)).await;
    todo!()
}
