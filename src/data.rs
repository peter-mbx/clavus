use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub configs: Vec<Config>,
    pub active: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub name: String,
    #[serde(default = "Vec::<File>::new")]
    pub files: Vec<File>,
    #[serde(default = "Vec::<Command>::new")]
    pub commands: Vec<Command>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub id: String,
    pub target: PathBuf,
    pub content: String,
    pub old_content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub id: String,
    pub up: String,
    pub down: String,
}
