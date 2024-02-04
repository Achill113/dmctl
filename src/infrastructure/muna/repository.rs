use crate::domain_shared::race::Race;
use crate::domain_shared::gender::Gender;
use super::client::MunaClient;

pub struct MunaRepository {

}

impl MunaRepository {
    pub async fn get_race_names(race: Race, gender: Gender, count: i16) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let result = MunaClient::get_names(race.to_string(), count, gender.to_string()).await?;

        Ok(result.names)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_race_names_should_return_10_elf_names_when_10_elf_names_is_requested() {
        let result = MunaRepository::get_race_names(Race::Elf, Gender::Male, 1).await;

        assert!(!result.is_err());
    }
}
