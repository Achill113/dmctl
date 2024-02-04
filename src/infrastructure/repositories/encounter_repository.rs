use std::collections::HashMap;
use std::error::Error;

use crate::domain::encounters::factories::encounter_factory::EncounterFactory;
use crate::infrastructure::results::encounter::{LevelRangeResult, RollRangeResult, EnemyFormulaResult};
use crate::domain::encounters::entities::encounter::BaseEncounter;
use crate::domain::repositories::encounter_repository::IEncounterRepository;
use crate::infrastructure::connection::establish_connection;
use crate::infrastructure::results::encounter::EncounterResult;
use crate::schema::*;

use diesel::prelude::*;

pub struct EncounterRepository {

}

impl IEncounterRepository for EncounterRepository {
    fn get_encounter(id: i32) -> Result<Box<dyn BaseEncounter>, Box<dyn Error>> {
        let connection = &mut establish_connection();

        let results = encounters::table
            .inner_join(level_range::table)
            .inner_join(roll_range::table)
            .left_join(enemy_formulas::table)
            .filter(encounters::id.eq(id))
            .select((EncounterResult::as_select(), LevelRangeResult::as_select(), RollRangeResult::as_select(), Option::<EnemyFormulaResult>::as_select()))
            .load::<(EncounterResult, LevelRangeResult, RollRangeResult, Option<EnemyFormulaResult>)>(connection)?;


        let mut grouped_results: HashMap<i32, (EncounterResult, LevelRangeResult, RollRangeResult, Vec<EnemyFormulaResult>)> = HashMap::new();

        for (encounter_result, level_range_result, roll_range_result, enemy_formula_result) in results {
            if let Some(result) = enemy_formula_result {
                grouped_results.entry(encounter_result.id)
                    .or_insert_with(|| (encounter_result, level_range_result, roll_range_result, Vec::new()))
                    .3
                    .push(result);
            }
        }


        let result = grouped_results.get(&id).expect("Encounter not found");
        let encounter_result = &result.0;
        let level_range_result = &result.1;
        let roll_range_result = &result.2;
        let enemy_formula_results = &result.3;

        let encounter = EncounterFactory::create_from_results(encounter_result, level_range_result, roll_range_result, enemy_formula_results)?;

        Ok(encounter)
    }
}
