use crate::{domain::encounters::{entities::{encounter::{BaseEncounter, CombatEncounter, Encounter, SituationalEncounter}, enemy_formula::EnemyFormula}, value_objects::{level_range::LevelRange, roll_range::RollRange}}, framework::result_loadable::ResultLoadable, infrastructure::results::encounter::{EncounterResult, EnemyFormulaResult, LevelRangeResult, RollRangeResult}};

pub struct EncounterFactory {}

impl EncounterFactory {
    pub fn create_from_results(encounter_result: &EncounterResult, level_range_result: &LevelRangeResult, roll_range_result: &RollRangeResult, enemy_formulas_results: &Vec<EnemyFormulaResult>) -> Result<Box<dyn BaseEncounter>, &'static str> {
        let level_range = LevelRange::create_from_result(&level_range_result);

        let roll_range = RollRange::create_from_result(&roll_range_result);

        let encounter = Encounter::new(encounter_result.environment.clone(), level_range, roll_range);

        if !enemy_formulas_results.is_empty() {
            let enemy_formulas = enemy_formulas_results.iter().map(|r| EnemyFormula::create_from_result(r)).collect::<Vec<EnemyFormula>>();

            let combat_encounter = CombatEncounter::new(encounter, enemy_formulas);

            return Ok(Box::new(combat_encounter));
        }

        if let Some(situation) = &encounter_result.situation {
            let situational_encounter = SituationalEncounter::new(encounter, situation.clone());

            return Ok(Box::new(situational_encounter));
        }
        
        return Err("An encounter must have either enemy formulas or a situation.");
    }
}
