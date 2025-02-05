use color_eyre::eyre;
use database::models::{Gardyn, GardynSlot, NewGardyn, NewGardynSlot, NewPlant, Plant};
use database::schema::{self};
use diesel::{insert_into, Connection, PgConnection, RunQueryDsl, SelectableHelper};

fn main() -> eyre::Result<()> {
    color_eyre::config::HookBuilder::default().install()?;

    dotenvy::dotenv()?;

    let conn = &mut establish_connection()?;

    let gardyns = insert_gardyns(conn, &["Leafy Castle", "Mary Jane"])?;

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

    let gardyn_slots = build_slots(conn, gardyns)?;

    let _plants = insert_plants_in_slot(conn, &gardyn_slots, &plant_names)?;

    Ok(())
}

fn insert_gardyns(
    conn: &mut PgConnection,
    gardyns_names: &[&str],
) -> diesel::result::QueryResult<Vec<Gardyn>> {
    let gardyns = gardyns_names
        .iter()
        .map(|name| NewGardyn { name })
        .collect::<Vec<_>>();

    insert_into(schema::gardyns::table)
        .values(&gardyns)
        .returning(Gardyn::as_returning())
        .get_results(conn)
}

fn build_slots(
    conn: &mut PgConnection,
    gardyns: Vec<Gardyn>,
) -> diesel::result::QueryResult<Vec<GardynSlot>> {
    let gardyn_slots = gardyns
        .into_iter()
        .flat_map(|g| {
            (0..3).flat_map(move |x| {
                (0..10).map(move |y| NewGardynSlot {
                    x,
                    y,
                    gardyn_id: g.id,
                    plant_id: None,
                })
            })
        })
        .collect::<Vec<_>>();

    insert_into(schema::gardyn_slots::table)
        .values(gardyn_slots)
        .returning(GardynSlot::as_returning())
        .get_results(conn)
}

fn insert_plants_in_slot(
    conn: &mut PgConnection,
    slots: &[GardynSlot],
    plant_names: &[&str],
) -> diesel::result::QueryResult<Vec<Plant>> {
    let creation = chrono::Local::now().naive_utc();
    let creation_offset = chrono::Local::now().offset().local_minus_utc();
    let creation_time_zone = "America/Phoenix";

    let plants_to_insert = plant_names
        .iter()
        .map(|name| NewPlant {
            name,
            creation,
            creation_offset,
            creation_time_zone,
        })
        .collect::<Vec<_>>();

    let plants = insert_into(schema::plants::table)
        .values(plants_to_insert)
        .returning(Plant::as_returning())
        .get_results(conn)?;

    for (_slot, _plant) in slots.iter().zip(plants.iter()) {

        // slot.plant_id = Some(plant.id);
    }

    Ok(plants)
}

fn establish_connection() -> eyre::Result<PgConnection> {
    let url = ::std::env::var("DATABASE_URL")?;

    PgConnection::establish(&url).map_err(Into::into)
}
