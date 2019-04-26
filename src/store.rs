use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use dirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    pub vars: BTreeMap<String, String>,
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.vars.len() == 0 {
            write!(f, "No vars yet for this project").expect("Failed to write to formatter");
            return Ok(());
        }

        for (key, value) in self.vars.iter() {
            write!(f, "{} = {}\n", key, value).expect("Failed to write project variable");
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct StoreFile {
    projects: BTreeMap<PathBuf, Project>,
}

#[derive(Debug)]
pub struct Store {
    store: StoreFile,
    file: fs::File,
    path: PathBuf,
}

impl Store {
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
                eprintln!(
                    "Error: Failed to parse config file. Please check that it is valid TOML."
                );
                std::process::exit(1);
            }
        };

        Store {
            store: store_file,
            file: file,
            path,
        }
    }

    pub fn add_project(&mut self, name: &PathBuf) -> Result<(), &str> {
        let new_project = Project {
            vars: BTreeMap::new(),
        };

        if self.store.projects.contains_key(name) {
            return Err("Key already exists in project list");
        }

        self.store
            .projects
            .insert(name.clone(), new_project.clone());
        Ok(self.write_projects())
    }

    pub fn get_project(&self, name: &PathBuf) -> Option<&Project> {
        self.store.projects.get(name)
    }

    pub fn set_var(&mut self, proj_name: &PathBuf, pair: &(&str, &str)) -> Result<(), ()> {
        let proj = match self.store.projects.get_mut(proj_name) {
            Some(proj) => proj,
            None => return Err(()),
        };

        proj.vars.insert(pair.0.to_string(), pair.1.to_string());
        Ok(self.write_projects())
    }

    fn create(path: &Path) -> Store {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.clone())
            .expect("Failed to create store file");

        let _ = file.write_all(
            &toml::to_vec(&StoreFile::default()).expect("Failed to serialize default StoreFile"),
        );

        let sf = StoreFile {
            projects: BTreeMap::new(),
        };

        Store {
            store: sf,
            file: file,
            path: path.to_path_buf(),
        }
    }

    fn write_projects(&mut self) -> () {
        let bytes = &toml::to_vec(&self.store).expect("Failed to serialize store for write");

        fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .expect("Failed to open config file")
            .write_all(bytes)
            .expect("Failed to write to disk, some data may be lost!");
    }
}
