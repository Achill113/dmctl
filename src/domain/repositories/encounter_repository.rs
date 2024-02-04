use crate::domain::encounters::entities::encounter::BaseEncounter;

pub trait IEncounterRepository {
    fn get_encounter(id: usize) -> Box<dyn BaseEncounter>;
}
