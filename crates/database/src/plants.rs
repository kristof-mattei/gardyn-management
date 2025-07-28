#![expect(clippy::todo, reason = "WIP")]

use color_eyre::eyre;
use diesel_async::AsyncPgConnection;

use crate::models::{GardynSlot, Plant};

pub async fn get_by_gardyn_id(
    _connection: &mut AsyncPgConnection,
    _id: i32,
) -> Result<Vec<(GardynSlot, Option<Plant>)>, eyre::Report> {
    todo!()
    // struct GardynPlantsSlots {
    //     gardyn_id: i32,
    //     slot_id: i32,
    //     x: i32,
    //     y: i32,
    //     plant_id: Option<i32>,
    //     name: Option<String>,
    //     creation: Option<NaiveDateTime>,
    //     creation_offset: Option<i32>,
    //     creation_time_zone: Option<String>,
    // }

    // let slots_with_plants = sqlx::query_as!(
    //     GardynPlantsSlots,
    //     r#"SELECT
    //         gardyn_slot.id as slot_id,
    //         gardyn_slot.gardyn_id as gardyn_id,
    //         gardyn_slot.x,
    //         gardyn_slot.y,
    //         plant.id as plant_id,
    //         plant.name,
    //         plant.creation,
    //         plant.creation_offset,
    //         plant.creation_time_zone
    //     FROM
    //         gardyn_slot
    //     LEFT JOIN
    //         plant ON gardyn_slot.plant_id = plant.id
    //     WHERE gardyn_slot.gardyn_id = $1"#,
    //     id
    // )
    // .fetch_all(pool)
    // .await?;

    // let mapped = slots_with_plants
    //     .into_iter()
    //     .map(
    //         |GardynPlantsSlots {
    //              gardyn_id,
    //              slot_id,
    //              x,
    //              y,
    //              plant_id,
    //              name,
    //              creation,
    //              creation_offset,
    //              creation_time_zone,
    //          }| {
    //             let slot = GardynSlot {
    //                 id: slot_id,
    //                 x,
    //                 y,
    //                 gardyn_id,
    //                 plant_id,
    //             };

    //             let plant = plant_id.map(|id| Plant {
    //                 id,
    //                 name: name.unwrap(),
    //                 creation: creation.unwrap(),
    //                 creation_offset: creation_offset.unwrap(),
    //                 creation_time_zone: creation_time_zone.unwrap(),
    //             });

    //             (slot, plant)
    //         },
    //     )
    //     .collect::<Vec<_>>();

    // Ok(mapped)
}
