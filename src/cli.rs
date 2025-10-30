use crate::iprintln;
use crate::plugin;
use crate::vprintln;
use clap::{Parser, Subcommand};
use strum::AsRefStr;

#[derive(Parser)]
#[command(name = "astrbot")]
#[command(about = "AstrBot CLI - A command line tool for managing AstrBot")]
#[command(version = "0.1.3")]
pub struct Cli {
    /// Enable verbose output with detailed logs and comprehensive information
    #[arg(short, long, global = true, help = "Enable verbose output")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Plugin management commands
    Plugin {
        #[command(subcommand)]
        action: PluginCommands,
    },
    /// Login to AstrBot
    Login {
        #[arg(short, long, help = "Username")]
        username: String,
        #[arg(short, long, help = "Password")]
        password: String,
        #[arg(short = 's', long = "server", help = "Server URL")]
        server: String,
    },
    /// Get astrbot stat
    Stat,
    /// Log-related commands
    Log {
        #[command(subcommand)]
        action: LogCommands,
    },
}

#[derive(Subcommand, AsRefStr)]
pub enum LogCommands {
    /// Get astrbot live log
    Live {
        /// flush logs, always print latest logs
        #[arg(short, long, help = "Flush logs")]
        flush: bool,
    },
    /// Get log history
    History {
        /// output file path
        #[arg(short, long, help = "Output file path")]
        output_file: String,
    },
}

#[derive(Subcommand, AsRefStr)]
pub enum PluginCommands {
    /// Get plugin list
    Get,
    /// Install a plugin
    Install {
        #[arg(long, help = "Install from local path")]
        from_local: bool,
        #[arg(long, help = "Install from git repository")]
        from_git: Option<String>,
    },
    /// Disable a plugin
    Off {
        #[arg(help = "Plugin name to disable")]
        plugin_name: String,
    },
    /// Enable a plugin
    On {
        #[arg(help = "Plugin name to enable")]
        plugin_name: String,
    },
    /// Reload plugins
    Reload {
        #[arg(help = "Plugin name to reload")]
        plugin_name: String,
    },
    /// Uninstall a plugin
    Uninstall {
        #[arg(help = "Plugin name to uninstall")]
        plugin_name: String,
    },
}

impl PluginCommands {
    pub fn plugin_name(&self) -> Option<&str> {
        match self {
            PluginCommands::Off { plugin_name }
            | PluginCommands::On { plugin_name }
            | PluginCommands::Reload { plugin_name }
            | PluginCommands::Uninstall { plugin_name } => Some(plugin_name),
            _ => None,
        }
    }
}

pub async fn handle_plugin_command(action: PluginCommands) {
    match action {
        PluginCommands::Get => {
            let plugings = super::plugin::handle_plugin_get().await;
            match plugings {
                Ok(plugings) => {
                    plugings.iter().for_each(|pl| pl.pretty_print());
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                }
            }
        }
        PluginCommands::Install {
            from_local,
            from_git,
        } => {
            if from_local {
                vprintln!("Installing plugin from current path");
                let ret = plugin::handle_plugin_install_from_local().await;
                match ret {
                    Ok(r) => {
                        iprintln!("{}", r);
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        std::process::exit(1);
                    }
                }
            } else if let Some(git_url) = from_git {
                vprintln!("Installing plugin from git repository: {}", git_url);
                let ret = plugin::handle_plugin_install_from_github(&git_url).await;
                match ret {
                    Ok(r) => {
                        iprintln!("{}", r);
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Error: Please specify either --from-local or --from-git option");
                std::process::exit(1);
            }
        }
        cmd @ (PluginCommands::Off { .. }
        | PluginCommands::Reload { .. }
        | PluginCommands::On { .. }
        | PluginCommands::Uninstall { .. }) => {
            let action = cmd.as_ref().to_ascii_lowercase();
            let plugin_name = cmd.plugin_name().unwrap();
            vprintln!("{} plugin: {}", action, plugin_name);
            let ret = plugin::handle_plugin_common_actions(plugin_name, action.as_str()).await;
            match ret {
                Ok(r) => {
                    iprintln!("{}", r);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                }
            }
        }
    }
}
