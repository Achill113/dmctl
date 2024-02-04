use std::error::Error;

use crate::{domain::{encounters::entities::encounter::BaseEncounter, repositories::encounter_repository::IEncounterRepository}, infrastructure::repositories::encounter_repository::EncounterRepository};

pub struct EncountersService {

}

impl EncountersService {
    pub fn get_encounter(id: i32) -> Result<Box<dyn BaseEncounter>, Box<dyn Error>> {
        EncounterRepository::get_encounter(id)
    }
}
