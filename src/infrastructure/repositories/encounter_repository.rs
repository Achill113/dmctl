use crate::domain::encounters::entities::encounter::{BaseEncounter, Encounter};
use crate::domain::encounters::value_objects::level_range::LevelRange;
use crate::domain::encounters::value_objects::roll_range::RollRange;
use crate::domain::repositories::encounter_repository::IEncounterRepository;

pub struct EncounterRepository {

}

impl IEncounterRepository for EncounterRepository {
    fn get_encounter(id: usize) -> Box<dyn BaseEncounter> {
        // TODO: Get an encounter from a database
        let level_range = LevelRange {
            min_level: 1,
            max_level: 20,
        };

        let roll_range = RollRange {
            min_roll: 1,
            max_roll: 100
        };

        Box::new(Encounter::new("test", &level_range, &roll_range))
    }
}
