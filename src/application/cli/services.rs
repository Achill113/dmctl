use crate::application::{encounters::services::EncountersService, names::services::NamesService};

use super::dtos::{CliDto, Commands, EncounterCommands, GenerateCommands};

pub struct CliService {

}

impl CliService {
    /// handles CLI input
    pub async fn handle_input(args: CliDto) -> String {
        match args.command {
            // generate
            Some(Commands::Generate { command }) => {
                Self::handle_generate_inputs(command).await
            },
            Some(Commands::Encounters { command }) => {
                Self::handle_encounter_inputs(command)
            },
            // no sub commands
            _ => {
                "No commands were provided".to_owned()
            }
        }
    }

    async fn handle_generate_inputs(command: Option<GenerateCommands>) -> String {
        match command {
            // generate name
            Some(GenerateCommands::Names { race, gender, count }) => {
                NamesService::get_names(race, gender, count).await
            },
            // generate
            _ => {
                "No Generate subcommands were provided".to_owned()
            }
        }
    }

    fn handle_encounter_inputs(command: Option<EncounterCommands>) -> String {
        match command {
            Some(EncounterCommands::Get { id }) => {
                match EncountersService::get_encounter(id) {
                    Ok(encounter) => encounter.environment().to_owned(),
                    Err(_) => "Something went wrong".to_owned()
                }
            },
            _ => {
                "No Encounter subcommands were provided".to_owned()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_shared::gender::Gender;
    use crate::domain_shared::race::Race;

    #[tokio::test]
    async fn handle_input_prints_names_when_generate_name_command_is_passed() {
        let args = CliDto {
            command: Some(Commands::Generate { command: Some(GenerateCommands::Names { race: Race::Elf, gender: Gender::Male, count: 10 } ) })
        };

        let result = CliService::handle_input(args).await;

        assert!(result.contains("Names:"))
    }
}
