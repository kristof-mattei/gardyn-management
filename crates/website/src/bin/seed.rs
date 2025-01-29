use color_eyre::eyre;
use database::models::{Gardyn, NewGardyn, NewPlant};
use database::schema::{self};
use diesel::{insert_into, Connection, PgConnection, RunQueryDsl, SelectableHelper};

fn main() -> eyre::Result<()> {
    color_eyre::config::HookBuilder::default().install()?;

    dotenvy::dotenv()?;

    let conn = &mut establish_connection()?;

    let gardyns = insert_gardyns(conn, &["Leafy Castle", "Mary Jane"])?;

    let creation = chrono::Local::now().naive_utc();
    let creation_offset = chrono::Local::now().offset().local_minus_utc();
    let creation_time_zone = "America/Phoenix";

    let plants_to_insert = &[
        NewPlant {
            name: "Romaine",
            gardyn_id: gardyns[0].id,
            x: 0,
            y: 0,
            creation,
            creation_offset,
            creation_time_zone,
        },
        NewPlant {
            name: "Banana Peppers",
            gardyn_id: gardyns[0].id,
            x: 0,
            y: 2,
            creation,
            creation_offset,
            creation_time_zone,
        },
        NewPlant {
            name: "Dill",
            gardyn_id: gardyns[0].id,
            x: 0,
            y: 5,
            creation,
            creation_offset,
            creation_time_zone,
        },
    ];

    insert_plants(conn, plants_to_insert)?;

    Ok(())
}

fn insert_gardyns(conn: &mut PgConnection, gardyns: &[&str]) -> eyre::Result<Vec<Gardyn>> {
    let gardyns = gardyns
        .iter()
        .map(|name| NewGardyn { name })
        .collect::<Vec<_>>();

    let results = insert_into(schema::gardyns::table)
        .values(&gardyns)
        .returning(Gardyn::as_returning())
        .get_results(conn)?;

    Ok(results)
}

fn insert_plants(conn: &mut PgConnection, plants: &[NewPlant]) -> eyre::Result<usize> {
    insert_into(schema::plants::table)
        .values(plants)
        .execute(conn)
        .map_err(Into::into)
}

fn establish_connection() -> eyre::Result<PgConnection> {
    let url = ::std::env::var("DATABASE_URL")?;

    PgConnection::establish(&url).map_err(Into::into)
}
