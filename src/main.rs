use std::env;
use std::path::{Path};

use clap::{load_yaml, App};

mod store;
mod utils;

const CONFIG_PATH: &'static str = ".emu.toml";

fn init_project(store: &mut store::Store) {
    let project_name = env::current_dir().expect("Failed to get current directory");

    match store.add_project(&project_name) {
        Ok(_) => println!("Successfully created new project: {}", utils::lossy(&project_name)),
        Err(e) => println!("{}", e),
    }

}

fn show_vars(store: &mut store::Store) {
    let project_name = env::current_dir().expect("Failed to get current directory");

    if let Some(p) = store.get_project(&project_name) {
        println!("{}", p);
    } else {
        println!("No project is initialized in the current dir: {}", utils::lossy(&project_name));
    }
}

fn show_vars_all(store: &mut store::Store) {
    unimplemented!()
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let store_path = match matches.value_of("store") {
        Some(s) => Path::new(s),
        None => Path::new(CONFIG_PATH),
    };

    let mut store = store::Store::open(store_path);

    match matches.subcommand() {
        ("init", Some(_sub_m)) => init_project(&mut store),
        ("show", Some(sub_m)) => {
            if sub_m.is_present("projects") {
                show_vars_all(&mut store)
            } else {
                show_vars(&mut store)
            }
        },
        _ => println!("{}", matches.usage()),
    };
}
