use std::{process::exit, path::{Path, self}};

use argparse::{ArgumentParser, Store, StoreTrue, StoreOption};
use env_logger::{Builder, Env};
use football_json_loader::{json::JsonLoader, db::DatabaseProcessor, model::{LeagueJsonFormat, LeagueMatch}};
use git2::Repository;
use log::{info, warn};

fn main() {
    let mut db_loc: Option<String> = None;
    let mut warning = false;
    let mut verbose = false;

    {
        let mut arg_parser = ArgumentParser::new();
        
        arg_parser.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Whether or not to show all logs");
        
        arg_parser.refer(&mut warning)
            .add_option(&["-w", "--warnings"], StoreTrue, "Whether or not to show logs (Warnings only)");

        arg_parser.refer(&mut db_loc)
            .add_option(&["-f", "--file_loc"], StoreOption, "Where to save the SQLite Database");
        
        arg_parser.parse_args_or_exit();
    }
    
    // If verbose, show logs
    if verbose {
        Builder::from_env(Env::default().default_filter_or("football_json_loader=trace"))
            .init();
    } else if warning {
        Builder::from_env(Env::default().default_filter_or("football_json_loader=warn"))
            .init();
    }
    info!("Initialized Logger");
    
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
    
    let _teams = match json_loader.load_teams() {
        Some(t) => t,
        None => {
            warn!("JsonLoader not yet initialized");
            exit(-1);
        }
    };
    
    // for t in teams.iter() {
    //     println!("{} {}", t.id, t.name);
    // }

    let _leagues = match json_loader.load_leagues() {
        Some(l) => l,
        None => {
            warn!("JsonLoader not yet initialized");
            exit(-1);
        }
    };
    
    // for league in leagues {
    //     match league {
    //         LeagueJsonFormat::OldFormat(old_league) => {
    //             for round in old_league.rounds.iter() {
    //                 for m in round.matches.iter() {
    //                     println!(
    //                         "{} Match: {}    {} v {} Score: {}:{}",
    //                         old_league.name,
    //                         m.get_date(),
    //                         m.get_team_1_name(),
    //                         m.get_team_2_name(),
    //                         m.get_team_1_score().unwrap_or(-1),
    //                         m.get_team_2_score().unwrap_or(-1)
    //                     );
    //                 }
    //             }
    //         },
    //         LeagueJsonFormat::NewFormat(new_league) => {
    //             for m in new_league.matches.iter() {
    //                 println!(
    //                     "{} Match: {}    {} v {} Score: {}:{}",
    //                     new_league.name,
    //                     m.get_date(),
    //                     m.get_team_1_name(),
    //                     m.get_team_2_name(),
    //                     m.get_team_1_score().unwrap_or(-1),
    //                     m.get_team_2_score().unwrap_or(-1)
    //                 );
    //             }
    //         }
    //     }
    // }
    
    // Saving the result to an SQLite database
    let mut db_processor = DatabaseProcessor::new();
    let db_init_result =  match db_loc {
        Some(path) => db_processor.init(&path),
        None => db_processor.init("football_json.sqlite")
    };
    
    if let Err(e) = db_init_result {
        warn!("Failed to initialize database: {}", e.to_string());
    }
    
    if let Err(e) = db_processor.insert_data_from_loader(&json_loader) {
        warn!("Failed to insert data into SQLite database: {}", e.to_string());
    }

}
