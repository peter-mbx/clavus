use crate::data;
use dirs::home_dir;
use once_cell::sync::Lazy;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Write};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
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

pub fn get_file_permissions(path: PathBuf) -> data::FilePermissions {
    let permissions = fs::metadata(path).unwrap().permissions();

    #[cfg(unix)]
    return data::FilePermissions {
        mode: Some(format!("{:o}", permissions.mode() & 0o777)),
        readonly: permissions.readonly(),
    };

    #[cfg(windows)]
    return data::FilePermissions {
        mode: None,
        readonly: permissions.readonly(),
    };
}

pub fn write_file(path: PathBuf, content: String, permissions: Option<data::FilePermissions>) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path.clone())
        .unwrap();
    let mut buf = BufWriter::new(file);
    write!(buf, "{}", content).unwrap();
    if permissions.is_some() {
        let mut perm = fs::metadata(path.clone()).unwrap().permissions();
        #[cfg(unix)]
        perm.set_mode(u32::from_str_radix(&permissions.unwrap().mode.unwrap(), 8).unwrap());
        #[cfg(windows)]
        perm.set_readonly(&permissions.unwrap().readonly);

        fs::set_permissions(path, perm).unwrap();
    }
}

pub fn write_state(state: data::State) {
    let res = serde_json::to_string_pretty(&state).unwrap();
    write_file((*CLAVUS_STATE_FILE).clone(), res, None);
}

pub fn init() {
    let _ = fs::create_dir_all(&*CLAVUS_DIR);

    if !(&*CLAVUS_STATE_FILE).to_path_buf().exists() {
        let confs = vec![data::Config {
            name: "default".to_string(),
            files: Vec::<data::File>::new(),
            commands: Vec::<data::Command>::new(),
        }];
        let state = data::State {
            configs: confs,
            active: "".to_string(),
        };
        write_state(state);
    }
}
