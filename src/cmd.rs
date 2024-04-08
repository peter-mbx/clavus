use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
#[group(required = true, multiple = false)]
pub enum ConfigCommands {
    #[command(arg_required_else_help = true, about = "create new configuration")]
    New {
        #[arg(long = "name", required = true)]
        name: String,
    },
    #[command(arg_required_else_help = true, about = "delete configuration")]
    Delete {
        #[arg(long = "name", required = true)]
        name: String,
    },
    #[command(arg_required_else_help = true, about = "add a file to configuration")]
    AddFile {
        #[arg(long = "config-name", required = true)]
        config_name: String,
        #[arg(long = "id", required = true)]
        id: String,
        #[arg(long = "source", required = true)]
        source: PathBuf,
        #[arg(long = "target", required = true)]
        target: PathBuf,
    },
    #[command(
        arg_required_else_help = true,
        about = "delete file from configuration"
    )]
    DeleteFile {
        #[arg(long = "config-name", required = true)]
        config_name: String,
        #[arg(long = "id", required = true)]
        id: String,
    },
    #[command(arg_required_else_help = true, about = "add command to configuration")]
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
        about = "delete command from configuration"
    )]
    DeleteCommand {
        #[arg(long = "config-name", required = true)]
        config_name: String,
        #[arg(long = "id", required = true)]
        id: String,
    },
    #[command(arg_required_else_help = true, about = "activate a configuration")]
    Up {
        #[arg(long = "name", required = true)]
        name: String,
    },
    #[command(about = "deactivate a configuration")]
    Down,
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
    #[command(arg_required_else_help = true, about = "manage configurations")]
    Config(ConfigArgs),
}

#[derive(Debug, Parser)]
#[command(name = "clavus", about = "Clavum Lateris", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: MainCommands,
}
