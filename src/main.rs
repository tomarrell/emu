use std::env;
use std::path::{Path};

use regex;
use clap::{load_yaml, App};

mod store;
mod utils;

const PAIR_REGEX: &'static str = r"\w+=\w+";
const CONFIG_PATH: &'static str = ".emu.toml";

fn init_project(store: &mut store::Store) {
    let proj_name = env::current_dir().expect("Failed to get current directory");

    match store.add_project(&proj_name) {
        Ok(_) => println!("Successfully created new project: {}", utils::lossy(&proj_name)),
        Err(e) => println!("{}", e),
    }
}

fn show_vars(store: &mut store::Store) {
    let proj_name = env::current_dir().expect("Failed to get current directory");

    if let Some(p) = store.get_project(&proj_name) {
        println!("{}", p);
    } else {
        println!("No project is initialized in the current dir: {}", utils::lossy(&proj_name));
    }
}

fn show_vars_all(store: &mut store::Store) {
    unimplemented!()
}

fn set_var(store: &mut store::Store, pair: (&str, &str)) {
    let proj_name = env::current_dir().expect("Failed to get current directory");
    match store.set_var(&proj_name, &pair) {
        Ok(_) => println!("Success"),
        Err(_) => println!("Not in a project. Please initialize the directory before setting vars."),
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut store = match matches.value_of("store") {
        Some(s) => store::Store::open(Path::new(s), false),
        None => store::Store::open(Path::new(CONFIG_PATH), true),
    };

    match matches.subcommand() {
        ("init", Some(_sub_m)) => init_project(&mut store),
        ("set", Some(sub_m)) => {
            if let Some(pair) = sub_m.value_of("var") {
                let re = regex::Regex::new(PAIR_REGEX).expect("Failed to create Regex");
                if re.is_match(pair) {
                    let pair: Vec<&str> = pair.split("=").collect();
                    set_var(&mut store, (pair[0], pair[1]));
                } else {
                    eprintln!("Invalid key pair, please correct format to: key=value");
                }
            }
        },
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
