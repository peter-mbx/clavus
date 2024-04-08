use base64::prelude::*;
use clap::Parser;
pub mod cmd;
pub mod core;
pub mod data;
pub mod util;

fn main() {
    core::init();

    let args = cmd::Cli::parse();

    match args.command {
        cmd::MainCommands::State => {
            core::show_state();
        }
        cmd::MainCommands::Status => {
            core::show_status();
        }
        cmd::MainCommands::List => {
            core::list();
        }
        cmd::MainCommands::Config(config) => match config.command {
            cmd::ConfigCommands::New { name } => {
                let conf = data::Config {
                    name,
                    files: Vec::<data::File>::new(),
                };
                core::create_conf(conf);
            }
            cmd::ConfigCommands::AddFile {
                config_name,
                id,
                source,
                target,
            } => {
                let file = data::File {
                    id,
                    target: target.clone(),
                    content: BASE64_STANDARD.encode(util::read_file(source)),
                    old_content: None,
                };
                core::add_file(config_name, file);
            }
            cmd::ConfigCommands::DeleteFile { config_name, id } => {
                core::delete_file(config_name, id);
            }
            cmd::ConfigCommands::Delete { name } => {
                core::delete_conf(name);
            }
            cmd::ConfigCommands::Up { name } => {
                core::activate_conf(name);
            }
            cmd::ConfigCommands::Down => {
                core::deactivate_conf();
            }
        },
    }
}
