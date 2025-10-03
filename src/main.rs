use clap::Parser;
use cli::{handle_plugin_command, Cli, Commands};
use login::handle_login;
use stat::handle_stat;

mod api;
mod cli;
mod config;
mod login;
mod plugin;
mod stat;
mod utils;
mod verbose;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize verbose mode
    verbose::init_verbose(cli.verbose);

    match cli.command {
        Commands::Plugin { action } => {
            handle_plugin_command(action).await;
        }
        Commands::Login {
            username,
            password,
            server,
        } => handle_login(username, password, server).await,
        Commands::Stat => {
            let ret = handle_stat().await;
            match ret {
                Ok(ret) => ret.pretty_print(),
                Err(e) => eprintln!("Error retrieving statistics: {}", e),
            }
        }
    }
}
