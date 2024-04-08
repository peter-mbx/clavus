use crate::data;
use dirs::home_dir;
use once_cell::sync::Lazy;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

pub static CLAVUS_DIR: Lazy<PathBuf> = Lazy::new(|| home_dir().unwrap().join(".clavus"));
pub static CLAVUS_STATE_FILE: Lazy<PathBuf> = Lazy::new(|| CLAVUS_DIR.join("state.json"));
pub static CLAVUS_STATE: Lazy<data::State> =
    Lazy::new(|| serde_json::from_str(&read_file(CLAVUS_STATE_FILE.to_path_buf())).unwrap());

pub fn read_file(path: PathBuf) -> String {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    return content;
}

pub fn write_file(path: PathBuf, content: String) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
    let mut buf = BufWriter::new(file);
    write!(buf, "{}", content).unwrap();
}

pub fn write_state(state: data::State) {
    let res = serde_json::to_string_pretty(&state).unwrap();
    write_file((*CLAVUS_STATE_FILE).clone(), res);
}
