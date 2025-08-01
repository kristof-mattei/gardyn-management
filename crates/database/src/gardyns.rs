use color_eyre::eyre;
use diesel::{QueryDsl as _, SelectableHelper as _};
use diesel_async::{AsyncPgConnection, RunQueryDsl as _};

use crate::models::Gardyn;
use crate::schema;

pub async fn get_all(connection: &mut AsyncPgConnection) -> Result<Vec<Gardyn>, eyre::Report> {
    schema::gardyn::table
        .select(Gardyn::as_select())
        .load(connection)
        .await
        .map_err(Into::into)
}

pub async fn one(connection: &mut AsyncPgConnection) -> Result<Vec<Gardyn>, eyre::Report> {
    schema::gardyn::table
        .select(Gardyn::as_select())
        .load(connection)
        .await
        .map_err(Into::into)
}
