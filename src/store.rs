use dirs;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
struct Project {
    vars: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct StoreFile {
    projects: BTreeMap<String, Project>,
}

#[derive(Debug)]
pub struct Store {
    store: StoreFile,
    file: fs::File,
    path: PathBuf,
}

impl Store{
    pub fn open(path: &Path) -> Store {
        let path = dirs::home_dir().expect("No home directory set").join(path);

        if !path.exists() {
            return Store::create(&path);
        }

        let mut buffer = String::new();
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("Failed to open store file");

        file.read_to_string(&mut buffer)
            .expect("Failed to read store file");

        let store_file: StoreFile = match toml::from_str(&buffer) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Error: Failed to parse config file. Please check that it is valid TOML.");
                std::process::exit(1);
            },
        };

        Store {
            store: store_file,
            file: file,
            path,
        }
    }

    fn create(path: &Path) -> Store {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.clone())
            .expect("Failed to create store file");

        let _ = file.write_all(b"projects:");

        let sf = StoreFile {
            projects: BTreeMap::new(),
        };

        Store {
            store: sf,
            file: file,
            path: path.to_path_buf(),
        }
    }
}
