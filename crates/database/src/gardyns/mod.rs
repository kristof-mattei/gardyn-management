use color_eyre::eyre;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::{self, Gardyn, GardynSlot, Plant};
use crate::schema;

pub async fn get_all(connection: &mut AsyncPgConnection) -> Result<Vec<Gardyn>, eyre::Report> {
    schema::gardyns::table
        .select(models::Gardyn::as_select())
        .load(connection)
        .await
        .map_err(Into::into)
}

pub async fn one(connection: &mut AsyncPgConnection) -> Result<Vec<Gardyn>, eyre::Report> {
    schema::gardyns::table
        .select(models::Gardyn::as_select())
        .load(connection)
        .await
        .map_err(Into::into)
}

pub async fn get_plants(
    connection: &mut AsyncPgConnection,
    id: i32,
) -> Result<Vec<(GardynSlot, Option<Plant>)>, eyre::Report> {
    schema::gardyn_slots::table
        .left_join(schema::plants::table)
        .filter(schema::gardyn_slots::gardyn_id.eq(id))
        .select((
            models::GardynSlot::as_select(),
            Option::<models::Plant>::as_select(),
        ))
        .load(connection)
        .await
        .map_err(Into::into)
}
