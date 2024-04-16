use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
#[group(required = true, multiple = false)]
pub enum ConfigCommands {
    #[command(arg_required_else_help = true, about = "create new configuration\n")]
    New {
        #[arg(long = "config-name", required = true)]
        config_name: String,
    },
    #[command(arg_required_else_help = true, about = "delete configuration\n")]
    Delete {
        #[arg(long = "config-name", required = true)]
        config_name: String,
    },
    #[command(
        arg_required_else_help = true,
        about = "add a file to configuration. if no option --source specified, open default editor\n"
    )]
    AddFile {
        #[arg(long = "config-name", required = true)]
        config_name: String,
        #[arg(long = "id", required = true)]
        id: String,
        #[arg(long = "source")]
        source: Option<PathBuf>,
        #[arg(long = "target", required = true)]
        target: PathBuf,
    },
    #[command(
        arg_required_else_help = true,
        about = "delete file from configuration\n"
    )]
    DeleteFile {
        #[arg(long = "config-name", required = true)]
        config_name: String,
        #[arg(long = "id", required = true)]
        id: String,
    },
    #[command(
        arg_required_else_help = true,
        about = "add command to configuration\n"
    )]
    AddCommand {
        #[arg(long = "config-name", required = true)]
        config_name: String,
        #[arg(long = "id", required = true)]
        id: String,
        #[arg(long = "up", required = true)]
        up: String,
        #[arg(long = "down", required = true)]
        down: String,
    },
    #[command(
        arg_required_else_help = true,
        about = "delete command from configuration\n"
    )]
    DeleteCommand {
        #[arg(long = "config-name", required = true)]
        config_name: String,
        #[arg(long = "id", required = true)]
        id: String,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Debug, Subcommand)]
pub enum MainCommands {
    #[command(about = "show state file")]
    State,
    #[command(about = "show current status")]
    Status,
    #[command(about = "list configurations")]
    List,
    #[command(arg_required_else_help = true, about = "activate a configuration")]
    Up {
        #[arg(long = "config-name", required = true)]
        config_name: String,
    },
    #[command(about = "deactivate a configuration")]
    Down,
    #[command(arg_required_else_help = true, about = "edit configurations")]
    Config(ConfigArgs),
}

#[derive(Debug, Parser)]
#[command(
    name = "clavus",
    about = "A lightweight command-line tool designed for anyone in the tech field, specially for IT consultants or professionals managing multiple clients environments.\nThis tool simplifies the management of configurations on your machine, allowing you to configure your environment and load the necessary resources, like files and/or commands.",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: MainCommands,
}
