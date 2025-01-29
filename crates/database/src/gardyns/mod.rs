use color_eyre::eyre;
use diesel::query_dsl::methods::SelectDsl;
use diesel::SelectableHelper;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::{self, Gardyn};
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
