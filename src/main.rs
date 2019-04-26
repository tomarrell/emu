use std::env;
use std::path::{Path, PathBuf};

use clap::{load_yaml, App};

mod store;

const CONFIG_PATH: &'static str = ".emu.toml";

fn get_project() -> PathBuf {
    return env::current_dir().expect("Failed to get current directory");
}

fn init(store: &mut store::Store) {
    let project = get_project();
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
        ("init", Some(_sub_m)) => init(&mut store),
        _ => println!("{}", matches.usage()),
    };
}
