use diesel::prelude::{Associations, Identifiable, Insertable, Queryable};
use diesel::Selectable;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = crate::schema::gardyns)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGardyn<'s> {
    pub name: &'s str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::plants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Gardyn))]
pub struct NewPlant<'n, 'ctz> {
    pub name: &'n str,
    pub x: i32,
    pub y: i32,
    pub gardyn_id: i32,
    pub creation: chrono::NaiveDateTime,
    pub creation_offset: i32,
    pub creation_time_zone: &'ctz str,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::gardyns)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Gardyn {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Associations, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::plants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Gardyn))]
pub struct Plant {
    pub id: i32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub gardyn_id: i32,
    // speciesId -> Int4,
    // creation -> Timestamp,
    // ending -> Timestamp,
}
