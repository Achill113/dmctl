use std::error::Error;

use crate::domain::encounters::entities::encounter::BaseEncounter;

pub trait IEncounterRepository {
    fn get_encounter(id: i32) -> Result<Box<dyn BaseEncounter>, Box<dyn Error>>;
}
