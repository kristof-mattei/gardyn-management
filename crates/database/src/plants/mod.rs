use color_eyre::eyre;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{BoolExpressionMethods, ExpressionMethods, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::{self, Plant};
use crate::schema;

pub async fn get_by_gardyn_id(
    connection: &mut AsyncPgConnection,
    id: i32,
) -> Result<Vec<Plant>, eyre::Report> {
    schema::plants::table
        .filter(
            schema::plants::gardyn_id
                .eq(id)
                .and(schema::plants::ending.is_null()),
        )
        .select(models::Plant::as_select())
        .load(connection)
        .await
        .map_err(Into::into)
}
