use std::{process::exit, path::Path};

use argparse::{ArgumentParser, Store, StoreTrue};
use env_logger::{Builder, Env};
use football_json_loader::json::JsonLoader;
use git2::Repository;
use log::{info, warn};

fn main() {
    let mut db_loc = String::new();
    let mut verbose = false;

    {
        let mut arg_parser = ArgumentParser::new();
        
        arg_parser.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Whether or not to show logs");

        arg_parser.refer(&mut db_loc)
            .add_option(&["-f", "--file_loc"], Store, "Where to save the SQLite Database");
        
        arg_parser.parse_args_or_exit();
    }
    
    // If verbose, show logs
    if verbose {
        Builder::from_env(Env::default().default_filter_or("football_json_loader=trace"))
            .init();
        info!("Initialized Logger");
    }
    
    // Checks if we have already cloned football.json. If not, clone it into /tmp
    let repo_path = Path::new("/tmp/football.json");
    if repo_path.exists() {
        info!("Using a cached copy of football.json...");
    } else {
        let url = "https://github.com/openfootball/football.json.git";
        
        if let Err(e) = Repository::clone(url, "/tmp/football.json") {
            warn!("Failed to clone football.json: {}", e);
            exit(-1);
        }
    }
    
    let mut json_loader = JsonLoader::new();
    if let Err(e) = json_loader.init() {
        warn!("Failed to initialize JsonLoader: {}", e.to_string());
        exit(-1);
    }
    
    let teams = match json_loader.load_teams() {
        Some(t) => t,
        None => {
            warn!("JsonLoader not yet initialized");
            exit(-1);
        }
    };
    
    for t in teams.iter() {
        println!("Team: {}, {}, {}", t.id, t.name, t.code.as_ref().unwrap_or(&String::from("NO CODE")));
    }
}
