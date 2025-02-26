use axum::extract::{Json, Path, State};
use axum::routing::{get, post};
use axum::Router;
use database::models::{Gardyn, GardynSlot, Plant};
use serde_json::json;

use crate::state::ApplicationState;
use crate::states::pool_wrapper::PoolWrapper;

pub fn build_api_router(state: ApplicationState) -> Router {
    Router::new()
        .route("/gardyns", get(gardyns))
        .route("/gardyn/{id}/plants", get(plants))
        .route("/plants/{plant_id}/move/{slot_id}", post(move_plant))
        .route(
            "/slots/{slot_id1}/swap/{slot_id2}",
            post(swap_plants_in_slots),
        )
        .route("/plants/{id}/pause", get(plants))
        .route("/plants/{id}/terminate", get(plants))
        .with_state(state)
}

async fn gardyns(State(PoolWrapper(pool)): State<PoolWrapper>) -> Json<Vec<Gardyn>> {
    // TODO fix unwrap
    let mut connection = pool.get().await.unwrap();

    let gardyns = database::gardyns::get_all(&mut connection).await.unwrap();

    Json(gardyns)
}

#[axum::debug_handler]
async fn plants(
    Path(id): Path<i32>,
    State(PoolWrapper(pool)): State<PoolWrapper>,
) -> Json<Vec<(GardynSlot, Option<Plant>)>> {
    // TODO fix unwrap
    let mut connection = pool.get().await.unwrap();

    let plants = database::plants::get_by_gardyn_id(&mut connection, id)
        .await
        .unwrap();

    Json(plants)
}

async fn move_plant(
    Path(plant_id): Path<i32>,
    Path(slot_id): Path<i32>,
    State(PoolWrapper(pool)): State<PoolWrapper>,
) -> Json<serde_json::Value> {
    let mut connection = pool.get().await.unwrap();
    // TODO: does the person invoking this own the plant?
    // does the person invoking this own the target slot_id?
    database::slots::move_plant_to_slot(&mut connection, plant_id, slot_id)
        .await
        .unwrap();

    Json(json!({
        "status": "ok",
    }))
}

async fn swap_plants_in_slots(
    Path(slot_id1): Path<i32>,
    Path(slot_id2): Path<i32>,
    State(PoolWrapper(pool)): State<PoolWrapper>,
) -> Json<serde_json::Value> {
    let mut connection = pool.get().await.unwrap();

    // TODO: does the person invoking both slots?
    database::slots::swap_plants_in_slots(&mut connection, slot_id1, slot_id2)
        .await
        .unwrap();

    Json(json!({
        "status": "ok",
    }))
}
