use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize)]
#[diesel(table_name = crate::schema::encounters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EncounterResult {
    pub id: i32,
    pub environment: String,
    pub situation: Option<String>,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(EncounterResult, foreign_key = encounter_id))]
#[diesel(table_name = crate::schema::enemy_formulas)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EnemyFormulaResult {
    pub id: i32,
    pub encounter_id: i32,
    pub enemy_name: String,
    pub die_type: Option<String>,
    pub count: i32,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(EncounterResult, foreign_key = encounter_id))]
#[diesel(table_name = crate::schema::level_range)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LevelRangeResult {
    pub id: i32,
    pub min_level: i32,
    pub max_level: i32,
    pub encounter_id: i32
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(EncounterResult, foreign_key = encounter_id))]
#[diesel(table_name = crate::schema::roll_range)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RollRangeResult {
    pub id: i32,
    pub min_roll: i32,
    pub max_roll: i32,
    pub encounter_id: i32
}

#[derive(Serialize)]
pub struct EncounterWithJoinsResult {
    #[serde(flatten)]
    encounter: EncounterResult,
    level_range: LevelRangeResult,
    roll_range: RollRangeResult,
    enemy_formulas: Vec<EnemyFormulaResult>
}
