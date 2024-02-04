use clap::Parser;

use crate::application::cli::services::CliService;
use crate::application::cli::dtos::CliDto;

mod application;
mod infrastructure;
mod domain;
mod domain_shared;
mod schema;
mod framework;

#[tokio::main]
async fn main() {
    let args = CliDto::parse();

    let result = CliService::handle_input(args).await;

    println!("{}", result);
}
