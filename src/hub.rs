use crate::util::{read_file, write_file, write_state, CLAVUS_STATE};
use crate::{data, util};
use base64::prelude::*;
use colored::Colorize;
use std::fs::remove_file;
use std::process::Command;
use which::which;

pub fn show_state() {
    let state = (*CLAVUS_STATE).clone();
    let res = serde_json::to_string_pretty(&state).unwrap();
    println!("{}", res);
}

pub fn show_status() {
    let state = (*CLAVUS_STATE).clone();
    if state.active != "" {
        println!("{} active", state.active.green());
    } else {
        println!("{}", "no active config".yellow());
    }
}

pub fn list() {
    let state = (*CLAVUS_STATE).clone();

    for conf in state.configs {
        if conf.name == state.active {
            println!("{}", conf.name.green());
        } else {
            println!("{}", conf.name);
        }
    }
}

pub fn activate_conf(name: String) {
    let mut state = (*CLAVUS_STATE).clone();
    if state.active == name {
        println!("{} already active", name.green());
        return;
    }
    if state.active != "".to_string() {
        println!("{} is active. deactivate it first", state.active.red());
        return;
    }
    let pos = state.configs.iter().position(|x| x.name == name);
    if !pos.is_some() {
        println!("{} does not exists", name.red());
        return;
    }
    let conf: &mut data::Config = &mut state.configs[pos.unwrap()];

    for c in &mut conf.commands {
        let mut full_command: Vec<String> = c.up.split(" ").map(|s| s.to_string()).collect();
        let command = full_command.remove(0);
        let command_path = which(command).unwrap();
        let _ = Command::new(command_path)
            .args(full_command)
            .spawn()
            .unwrap()
            .wait();
    }

    for f in &mut conf.files {
        if f.target.exists() {
            f.old_content = Some(BASE64_STANDARD.encode(read_file(f.target.clone())));
            f.old_permissions = Some(util::get_file_permissions(f.target.clone()));
        }
        write_file(
            f.target.clone(),
            std::str::from_utf8(&BASE64_STANDARD.decode(f.content.clone()).unwrap())
                .unwrap()
                .to_string(),
            Some(f.permissions.clone()),
        );
    }

    state.active = name.clone();
    write_state(state);
    println!("{} activated", name.yellow());
}

pub fn deactivate_conf() {
    let mut state = (*CLAVUS_STATE).clone();
    if state.active == "".to_string() {
        println!("{}", "no active config".yellow());
        return;
    }

    let pos = state.configs.iter().position(|x| x.name == state.active);
    let conf: &mut data::Config = &mut state.configs[pos.unwrap()];

    for c in &mut conf.commands {
        let mut full_command: Vec<String> = c.down.split(" ").map(|s| s.to_string()).collect();
        let command = full_command.remove(0);
        let command_path = which(command).unwrap();
        let _ = Command::new(command_path)
            .args(full_command)
            .spawn()
            .unwrap()
            .wait();
    }

    for f in &mut conf.files {
        if f.old_content.is_some() {
            write_file(
                f.target.clone(),
                std::str::from_utf8(
                    &BASE64_STANDARD
                        .decode(f.old_content.clone().unwrap())
                        .unwrap(),
                )
                .unwrap()
                .to_string(),
                f.old_permissions.clone(),
            );
            f.old_content = None;
        } else {
            let _ = remove_file(f.target.clone()).unwrap();
        }
        if f.old_permissions.is_some() {
            f.old_permissions = None;
        }
    }

    state.active = "".to_string();
    write_state(state);
    println!("{}", "deactivated".yellow());
}

pub fn create_conf(conf: data::Config) {
    let mut state = (*CLAVUS_STATE).clone();
    let config_name = conf.name.clone();
    let pos = state.configs.iter().position(|x| x.name == config_name);
    if pos.is_some() {
        println!("{} already exists", config_name.red());
        return;
    }
    state.configs.push(conf);
    write_state(state);
    println!("{} created", config_name.yellow());
}

pub fn delete_conf(name: String) {
    let mut state = (*CLAVUS_STATE).clone();
    let pos = state.configs.iter().position(|x| x.name == name);
    if !pos.is_some() {
        println!("{} does not exists", name.red());
        return;
    }

    state.configs.retain(|x| x.name != name);
    if state.active == name {
        state.active = "".to_string()
    }
    write_state(state);
    println!("{} deleted", name.yellow());
}

pub fn add_file(config_name: String, file: data::File) {
    let mut state = (*CLAVUS_STATE).clone();
    if state.active == config_name {
        println!("{} is active. deactivate it first", config_name.green());
        return;
    }
    let pos = state.configs.iter().position(|x| x.name == config_name);
    if !pos.is_some() {
        println!("{} does not exists", config_name.red());
        return;
    }
    let conf: &mut data::Config = &mut state.configs[pos.unwrap()];
    let fexists = conf.files.iter().position(|x| x.id == file.id);
    if fexists.is_some() {
        println!(
            "file {} in {} already exists",
            file.id.red(),
            config_name.green()
        );
        return;
    }
    conf.files.push(file.clone());
    write_state(state);
    println!("file {} added in {}", file.id.yellow(), config_name.green());
}

pub fn delete_file(config_name: String, file_id: String) {
    let mut state = (*CLAVUS_STATE).clone();
    if state.active == config_name {
        println!("{} is active. deactivate it first", config_name.red());
        return;
    }
    let pos = state.configs.iter().position(|x| x.name == config_name);
    if !pos.is_some() {
        println!("{} does not exists", config_name.red());
        return;
    }
    let conf: &mut data::Config = &mut state.configs[pos.unwrap()];
    let fexists = conf.files.iter().position(|x| x.id == file_id);
    if !fexists.is_some() {
        println!(
            "file {} in {} does not exists",
            file_id.red(),
            config_name.green()
        );
        return;
    }
    conf.files.retain(|x| x.id != file_id);
    write_state(state);
    println!(
        "file {} in {} deleted",
        file_id.yellow(),
        config_name.green()
    );
}

pub fn add_command(config_name: String, command: data::Command) {
    let mut state = (*CLAVUS_STATE).clone();
    if state.active == config_name {
        println!("{} is active. deactivate it first", config_name.green());
        return;
    }
    let pos = state.configs.iter().position(|x| x.name == config_name);
    if !pos.is_some() {
        println!("{} does not exists", config_name.red());
        return;
    }
    let conf: &mut data::Config = &mut state.configs[pos.unwrap()];
    let cexists = conf.commands.iter().position(|x| x.id == command.id);
    if cexists.is_some() {
        println!(
            "command {} in {} already exists",
            command.id.red(),
            config_name.green()
        );
        return;
    }
    conf.commands.push(command.clone());
    write_state(state);
    println!(
        "command {} added in {}",
        command.id.yellow(),
        config_name.green()
    );
}

pub fn delete_command(config_name: String, command_id: String) {
    let mut state = (*CLAVUS_STATE).clone();
    if state.active == config_name {
        println!("{} is active. deactivate it first", config_name.red());
        return;
    }
    let pos = state.configs.iter().position(|x| x.name == config_name);
    if !pos.is_some() {
        println!("{} does not exists", config_name.red());
        return;
    }
    let conf: &mut data::Config = &mut state.configs[pos.unwrap()];
    let cexists = conf.commands.iter().position(|x| x.id == command_id);
    if !cexists.is_some() {
        println!(
            "command {} in {} already exists",
            command_id.red(),
            config_name.green()
        );
        return;
    }
    conf.commands.retain(|x| x.id != command_id);
    write_state(state);
    println!(
        "command {} in {} deleted",
        command_id.yellow(),
        config_name.green()
    );
}
