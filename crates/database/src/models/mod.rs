use chrono::NaiveDateTime;
use diesel::Selectable;
use diesel::prelude::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = crate::schema::gardyn)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGardyn<'s> {
    pub name: &'s str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::gardyn_slot)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Gardyn))]
pub struct NewGardynSlot {
    pub x: i32,
    pub y: i32,
    pub gardyn_id: i32,
    pub plant_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::plant)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPlant<'n, 'ctz> {
    pub name: &'n str,
    pub creation: chrono::NaiveDateTime,
    pub creation_offset: i32,
    pub creation_time_zone: &'ctz str,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::gardyn)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Gardyn {
    pub id: i32,
    pub name: String,
}

#[derive(
    Queryable, Selectable, AsChangeset, Associations, Identifiable, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::gardyn_slot)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Gardyn))]
#[diesel(belongs_to(Plant))]
pub struct GardynSlot {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub gardyn_id: i32,
    pub plant_id: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::plant)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Plant {
    pub id: i32,
    pub name: String,
    // speciesId -> Int4,
    pub creation: NaiveDateTime,
    pub creation_offset: i32,
    pub creation_time_zone: String,
}
