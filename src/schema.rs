// @generated automatically by Diesel CLI.

diesel::table! {
    encounters (id) {
        id -> Integer,
        environment -> Text,
        situation -> Nullable<Text>,
    }
}

diesel::table! {
    enemy_formulas (id) {
        id -> Integer,
        encounter_id -> Integer,
        enemy_name -> Text,
        die_type -> Nullable<Text>,
        count -> Integer,
    }
}

diesel::table! {
    level_range (id) {
        id -> Integer,
        min_level -> Integer,
        max_level -> Integer,
        encounter_id -> Integer,
    }
}

diesel::table! {
    roll_range (id) {
        id -> Integer,
        min_roll -> Integer,
        max_roll -> Integer,
        encounter_id -> Integer,
    }
}

diesel::joinable!(enemy_formulas -> encounters (encounter_id));
diesel::joinable!(level_range -> encounters (encounter_id));
diesel::joinable!(roll_range -> encounters (encounter_id));

diesel::allow_tables_to_appear_in_same_query!(
    encounters,
    enemy_formulas,
    level_range,
    roll_range,
);
