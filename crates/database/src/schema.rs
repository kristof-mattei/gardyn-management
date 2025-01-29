// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    gardyns (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    plants (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        x -> Int4,
        y -> Int4,
        gardyn_id -> Int4,
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

diesel::joinable!(plants -> gardyns (gardyn_id));
diesel::joinable!(plants -> species (species_id));
diesel::joinable!(species -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(categories, gardyns, plants, species,);
