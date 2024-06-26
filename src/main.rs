use base64::prelude::*;
use clap::Parser;
pub mod cmd;
pub mod data;
pub mod hub;
pub mod util;
use edit::edit;

fn main() {
    util::init();

    let args = cmd::Cli::parse();

    match args.command {
        cmd::MainCommands::State => {
            hub::show_state();
        }
        cmd::MainCommands::Status => {
            hub::show_status();
        }
        cmd::MainCommands::List => {
            hub::list();
        }
        cmd::MainCommands::Up { config_name } => {
            hub::activate_conf(config_name);
        }
        cmd::MainCommands::Down => {
            hub::deactivate_conf();
        }
        cmd::MainCommands::Config(config) => match config.command {
            cmd::ConfigCommands::New { config_name } => {
                let conf = data::Config {
                    name: config_name,
                    files: Vec::<data::File>::new(),
                    commands: Vec::<data::Command>::new(),
                };
                hub::create_conf(conf);
            }
            cmd::ConfigCommands::Delete { config_name } => {
                hub::delete_conf(config_name);
            }
            cmd::ConfigCommands::AddFile {
                config_name,
                id,
                source,
                target,
            } => {
                let content;
                let permissions;

                if source.is_some() {
                    content = BASE64_STANDARD
                        .encode(util::read_file(util::tilde(source.clone().unwrap(), false)));
                    permissions =
                        util::get_file_permissions(util::tilde(source.clone().unwrap(), false));
                } else {
                    let res = edit(String::new()).unwrap();
                    content = BASE64_STANDARD.encode(res);
                    permissions = data::FilePermissions {
                        mode: Some("644".to_string()),
                        readonly: false,
                    };
                }

                let file = data::File {
                    id,
                    target: util::tilde(target, true),
                    content,
                    old_content: None,
                    permissions,
                    old_permissions: None,
                };
                hub::add_file(config_name, file);
            }
            cmd::ConfigCommands::DeleteFile { config_name, id } => {
                hub::delete_file(config_name, id);
            }
            cmd::ConfigCommands::AddCommand {
                config_name,
                id,
                up,
                down,
            } => {
                let command = data::Command { id, up, down };

                hub::add_command(config_name, command);
            }
            cmd::ConfigCommands::DeleteCommand { config_name, id } => {
                hub::delete_command(config_name, id);
            }
        },
    }
}
