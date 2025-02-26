// @generated automatically by Diesel CLI.

diesel::table! {
    category (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    gardyn (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    gardyn_slot (id) {
        id -> Int4,
        gardyn_id -> Int4,
        plant_id -> Nullable<Int4>,
        x -> Int4,
        y -> Int4,
    }
}

diesel::table! {
    plant (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        species_id -> Nullable<Int4>,
        creation -> Timestamp,
        creation_offset -> Int4,
        creation_time_zone -> Text,
        ending -> Nullable<Timestamp>,
        ending_offset -> Nullable<Int4>,
        ending_time_zone -> Nullable<Text>,
    }
}

diesel::table! {
    species (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        category_id -> Int4,
    }
}

diesel::joinable!(gardyn_slot -> gardyn (gardyn_id));
diesel::joinable!(gardyn_slot -> plant (plant_id));
diesel::joinable!(plant -> species (species_id));
diesel::joinable!(species -> category (category_id));

diesel::allow_tables_to_appear_in_same_query!(category, gardyn, gardyn_slot, plant, species,);
