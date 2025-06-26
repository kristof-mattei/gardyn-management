#![expect(clippy::unused_async)]

use color_eyre::eyre;
use database::models::{Gardyn, GardynSlot, Plant};
use database::schema::{gardyn, gardyn_slot, plant};
use diesel::{ExpressionMethods, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::config::HookBuilder::default().install()?;

    dotenvy::dotenv()?;

    let mut connection = establish_connection().await?;

    let gardyns = insert_gardyns(&mut connection, &["Leafy Castle", "Mary Jane"]).await?;

    let plant_names = [
        "Banana Peppers",
        "Basil (ours)",
        "Bok Choy Green",
        "Breen",
        "Cauliflower Mini",
        "Celery",
        "Cherry Tomato Red",
        "Cherry Tomato Yellow",
        "Cilantro",
        "Collard Greens",
        "Dill",
        "Jalapeno",
        "Jalapeno",
        "Kale Classic",
        "Kale Lacinato",
        "Kohlrabi Purple",
        "Lemon Balm",
        "Onion Bunching",
        "Onion Bunching",
        "Perpetual Spinach",
        "Perpetual Spinach",
        "Purple Kohlrabi",
        "Salanova Green",
        "Shishito",
        "Sweet Basil",
        "Sweet Thai Basil",
        "Tatsoi Red",
        "Thyme",
        "Tokyo Bekana",
        "Yellow Swiss Chard",
    ];

    let gardyn_slots = build_slots(&mut connection, gardyns).await?;

    let plants = insert_plants(&mut connection, &plant_names).await?;

    insert_plants_in_slot(&mut connection, &gardyn_slots, &plants).await?;

    Ok(())
}

async fn insert_plants(
    connection: &mut AsyncPgConnection,
    plant_names: &[&str],
) -> eyre::Result<Vec<Plant>> {
    let creation = chrono::Local::now().naive_utc();
    let creation_offset = chrono::Local::now().offset().local_minus_utc();
    let creation_time_zone = "America/Phoenix";

    let plants = plant_names
        .iter()
        .map(|g| {
            (
                plant::name.eq(g),
                plant::creation.eq(creation),
                plant::creation_offset.eq(creation_offset),
                plant::creation_time_zone.eq(creation_time_zone),
            )
        })
        .collect::<Vec<_>>();

    let plants = diesel::insert_into(plant::table)
        .values(plants)
        .returning(Plant::as_returning())
        .load(connection)
        .await?;

    Ok(plants)
}

async fn insert_gardyns(
    connection: &mut AsyncPgConnection,
    gardyn_names: &[&str],
) -> eyre::Result<Vec<Gardyn>> {
    let gardyns = diesel::insert_into(gardyn::table)
        .values(
            gardyn_names
                .iter()
                .map(|n| gardyn::name.eq(n))
                .collect::<Vec<_>>(),
        )
        .returning(Gardyn::as_returning())
        .load(connection)
        .await?;

    Ok(gardyns)
}

async fn build_slots(
    connection: &mut AsyncPgConnection,
    gardyns: Vec<Gardyn>,
) -> eyre::Result<Vec<GardynSlot>> {
    let slots = gardyns
        .into_iter()
        .flat_map(|g| {
            (0..3).flat_map(move |x| {
                (0..10).map(move |y| {
                    (
                        gardyn_slot::x.eq(x),
                        gardyn_slot::y.eq(y),
                        gardyn_slot::gardyn_id.eq(g.id),
                    )
                })
            })
        })
        .collect::<Vec<_>>();

    let slots = diesel::insert_into(gardyn_slot::table)
        .values(slots)
        .returning(GardynSlot::as_returning())
        .load(connection)
        .await?;

    Ok(slots)
}

async fn insert_plants_in_slot(
    _connection: &mut AsyncPgConnection,
    slots: &[GardynSlot],
    plants: &[Plant],
) -> eyre::Result<()> {
    let _slot_ids = slots.iter().map(|slot| slot.id).collect::<Vec<_>>();
    let _plant_ids = plants.iter().map(|plant| plant.id).collect::<Vec<_>>();

    // let updated_row = diesel::update(users.filter(id.eq(1)))
    //     .set((name.eq("James"), surname.eq("Bond")))
    //     .get_result(connection);
    // diesel::update(gard)

    // // RETURNING id, x, y, gardyn_id, plant_id
    // sqlx::query_as!(
    //     GardynSlot,
    //     r#"UPDATE
    //         gardyn_slot
    //     SET plant_id = u.plant_id
    //         FROM (SELECT * FROM UNNEST($1::int4[], $2::int4[])) AS u(slot_id, plant_id)
    //     WHERE
    //         u.slot_id = gardyn_slot.id"#,
    //     slot_ids as _,
    //     plant_ids as _,
    // )
    // .fetch_all(connection)
    // .await?;
    Ok(())
}

async fn establish_connection() -> eyre::Result<AsyncPgConnection> {
    Ok(AsyncPgConnection::establish(&std::env::var("DATABASE_URL")?).await?)
}
