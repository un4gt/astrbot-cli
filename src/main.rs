use clap::Parser;
use cli::{handle_plugin_command, Cli, Commands, LogCommands};
use log::{handle_history_log, handle_live_log};

use login::handle_login;
use stat::handle_stat;

mod api;
mod cli;
mod config;
mod log;
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
        Commands::Log { action } => match action {
            LogCommands::Live { flush } => {
                if let Err(err) = handle_live_log(flush).await {
                    eprintln!("Error fetching live log: {}", err);
                    std::process::exit(1);
                }
            }
            LogCommands::History { output_file } => {
                if let Err(err) = handle_history_log(output_file).await {
                    eprintln!("Error fetching log history: {}", err);
                    std::process::exit(1);
                }
            }
        },
    }
}
