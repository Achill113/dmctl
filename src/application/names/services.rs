use crate::{domain_shared::{gender::Gender, race::Race}, infrastructure::muna::repository::MunaRepository};

pub struct NamesService {

}

impl NamesService {
    pub async fn get_names(race: Race, gender: Gender, count: i16) -> String {
        let response = MunaRepository::get_race_names(race, gender, count).await.expect("Something went wrong generating names");

        let names = response.join(", ");

        format!("Names: {}", names)
    }
}
