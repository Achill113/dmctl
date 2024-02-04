use crate::{application::names::services::NamesService, infrastructure::muna::repository::MunaRepository};

use super::dtos::{CliDto, Commands, GenerateCommands};

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
