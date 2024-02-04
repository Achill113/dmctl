use clap::{Parser, Subcommand};

use crate::domain_shared::{gender::Gender, race::Race};

#[derive(Subcommand, Debug)]
pub enum GenerateCommands {
    Names {
        #[arg(short, long)]
        race: Race,

        #[arg(short, long)]
        gender: Gender,

        #[arg(short, long, default_value_t = 1)]
        count: i16,
    }
}

#[derive(Subcommand, Debug)]
pub enum EncounterCommands {
    Get {
        #[arg(short, long)]
        id: i32
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Generate {
        #[command(subcommand)]
        command: Option<GenerateCommands>
    },
    Encounters {
        #[command(subcommand)]
        command: Option<EncounterCommands>
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliDto {
    /// The section of commands to run [generate, roll]
    #[command(subcommand)]
    pub command: Option<Commands>,
}
