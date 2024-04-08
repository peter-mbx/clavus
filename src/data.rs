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
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub id: String,
    pub target: PathBuf,
    pub content: String,
    pub old_content: Option<String>,
}
