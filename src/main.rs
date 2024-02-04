use clap::Parser;

use crate::application::cli::services::CliService;
use crate::application::cli::dtos::CliDto;

mod application;
mod infrastructure;
mod domain;
mod domain_shared;

#[tokio::main]
async fn main() {
    let args = CliDto::parse();

    let result = CliService::handle_input(args).await;

    println!("{}", result);
}
