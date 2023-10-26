use log::{warn, info};
use rusqlite::{Connection, Result, Error, params};

use crate::{json::JsonLoader, model::{LeagueJsonFormat, LeagueMatch}};

pub struct DatabaseProcessor {
    conn: Option<Connection>,
    initialized: bool,
}

impl DatabaseProcessor {
    pub fn new() -> Self {
        DatabaseProcessor { conn: None, initialized: false }
    }

    pub fn init(&mut self, path: &str) -> Result<()> {
        self.conn = Some(Connection::open(path)?);
        
        self.create_teams_table()?;
        self.create_leagues_table()?;
        self.create_matches_table()?;
        
        self.initialized = true;

        Ok(())
    }
    
    fn create_teams_table(&self) -> Result<()> {
        if let Some(conn) = &self.conn {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS teams (
                    id      INTEGER PRIMARY KEY,
                    name    TEXT    NOT NULL,
                    code    TEXT
                )", 
                ()
            )?;
        }
        
        Ok(())
    }
    
    fn create_leagues_table(&self) -> Result<()> {
        if let Some(conn) = &self.conn {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS leagues (
                    id      INTEGER PRIMARY KEY,
                    name    TEXT    NOT NULL
                )",
                ()
            )?;
            
        }
        
        Ok(())
    }
    
    fn create_matches_table(&self) -> Result<()> {
        if let Some(conn) = &self.conn {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS matches (
                    id          INTEGER PRIMARY KEY,
                    date        TEXT NOT NULL,
                    league_id   INTEGER NOT NULL REFERENCES leagues(id),
                    team1_id    INTEGER NOT NULL REFERENCES teams(id),
                    team2_id    INTEGER NOT NULL REFERENCES teams(id),
                    stage       TEXT,
                    round       TEXT,
                    team1_score INTEGER,
                    team2_score INTEGER
                )",
                ()
            )?;
        }
        
        Ok(())
    }
    
    /// Inserts all data from a JsonLoader object into the SQLite database
    pub fn insert_data_from_loader(&self, json_loader: &JsonLoader) -> Result<()> {
        if !self.initialized {
            warn!("DatabaseProcessor not yet initialized. Please initialize this object first.");
            return Err(Error::InvalidQuery);
        }

        self.insert_teams_from_loader(json_loader)?;
        self.insert_leagues_from_loader(json_loader)?;
        self.insert_matches_from_loader(json_loader)?;
        
        Ok(())
    }
    
    /// Iterates through all teams loaded into the JsonLoader and inserts them
    /// into the SQLite database
    fn insert_teams_from_loader(&self, json_loader: &JsonLoader) -> Result<()> {
        info!("Inserting Teams...");
        let team_list = json_loader.get_teams();
        let conn = match &self.conn {
            Some(c) => c,
            None => {
                warn!("Connection not established. Please initialize the object first.");
                return Err(Error::InvalidQuery);
            }
        };
        
        for team in team_list.iter() {
            let insert_result = conn.execute(
                "INSERT INTO teams (id, name, code)
                VALUES (?1, ?2, ?3)",
                params![
                    team.id,
                    team.name,
                    team.code
                ]
            );
            
            if let Err(e) = insert_result {
                warn!("Failed to insert team: {}", e.to_string());
                warn!("Data: {} {} {}", team.id.to_string(), team.name, team.code.clone().unwrap_or("NONE".to_string()));
                continue;
            }
        }

        Ok(())
    }
    
    /// Iterates through all leagues and inserts them into the SQLite database.
    fn insert_leagues_from_loader(&self, json_loader: &JsonLoader) -> Result<()> {
        info!("Inserting Leagues...");
        let leagues = json_loader.get_leagues();
        let conn = match &self.conn {
            Some(c) => c,
            None => {
                warn!("Connection not established. Please initialize the object first.");
                return Err(Error::InvalidQuery);
            }
        };
        
        for league in leagues.iter() {
            let insert_res = conn.execute(
                "INSERT INTO leagues (name)
                VALUES (?1)",
                &[&league.get_league_name()]
            );
            
            if let Err(e) = insert_res {
                warn!("Failed to insert league: {}", e.to_string());
                warn!("Data: {}", league.get_league_name());
                continue;
            }
        }

        Ok(())
    }
    
    /// Iterates through all matches and inserts them into the SQLite database.
    fn insert_matches_from_loader(&self, json_loader: &JsonLoader) -> Result<()> {
        info!("Inserting matches...");
        let leagues = json_loader.get_leagues();
        let conn = match &self.conn {
            Some(c) => c,
            None => {
                warn!("Connection not established. Please initialize the object first.");
                return Err(Error::InvalidQuery);
            }
        };

        for league in leagues.iter() {
            match league {
                LeagueJsonFormat::OldFormat(old_league) => {
                    let league_id = old_league.id;
                    let stage: Option<String> = None;
                    
                    for round in old_league.rounds.iter() {
                        let round_name = &round.name;
                        
                        for m in round.matches.iter() {
                            let match_date = m.get_date();
                            let team1_id = json_loader.get_team_id_from_name(&m.team1);
                            let team2_id = json_loader.get_team_id_from_name(&m.team2);
                            let team1_score = m.get_team_1_score();
                            let team2_score = m.get_team_2_score();
                            
                            if let None = team1_id {
                                warn!("Skipping match data...");
                                warn!("Data: {} {} {} {} {} {} {} {}", match_date, league_id, team1_id.unwrap_or(-1), team2_id.unwrap_or(-1), stage.clone().unwrap_or("".to_string()), round_name, team1_score.unwrap_or(-1), team2_score.unwrap_or(-1));
                                continue;
                            }
                            
                            if let None = team2_id {
                                warn!("Skipping match data...");
                                warn!("Data: {} {} {} {} {} {} {} {}", match_date, league_id, team1_id.unwrap_or(-1), team2_id.unwrap_or(-1), stage.clone().unwrap_or("".to_string()), round_name, team1_score.unwrap_or(-1), team2_score.unwrap_or(-1));
                                continue;
                            }
                            
                            let insert_res = conn.execute(
                                "INSERT INTO matches (date, league_id, team1_id, team2_id, stage, round, team1_score, team2_score)
                                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                                params![
                                    match_date,
                                    league_id,
                                    team1_id.unwrap_or(-1),
                                    team2_id.unwrap_or(-1),
                                    stage.clone(),
                                    round_name,
                                    team1_score,
                                    team2_score
                                ]
                            );
                            
                            if let Err(e) = insert_res {
                                warn!("Failed to insert match: {}", e.to_string());
                                warn!("Data: {} {} {} {} {} {} {} {}", match_date, league_id, team1_id.unwrap_or(-1), team2_id.unwrap_or(-1), stage.clone().unwrap_or("".to_string()), round_name, team1_score.unwrap_or(-1), team2_score.unwrap_or(-1));
                                continue;
                            }
                        }
                    }
                },
                LeagueJsonFormat::NewFormat(new_league) => {
                    let league_id = new_league.id;
                    
                    for m in new_league.matches.iter() {
                        let stage = m.stage.clone();
                        let round_name = m.round.clone();
                        let match_date = m.get_date();
                        let team1_id = json_loader.get_team_id_from_name(&m.get_team_1_name().to_string());
                        let team2_id = json_loader.get_team_id_from_name(&m.get_team_2_name().to_string());
                        let team1_score = m.get_team_1_score();
                        let team2_score = m.get_team_2_score();
                        
                        if let None = team1_id {
                            warn!("Skipping match data...");
                            warn!("Data: {} {} {} {} {} {} {} {}", match_date, league_id, team1_id.unwrap_or(-1), team2_id.unwrap_or(-1), stage.clone().unwrap_or("".to_string()), round_name, team1_score.unwrap_or(-1), team2_score.unwrap_or(-1));
                            continue;
                        }
                        
                        if let None = team2_id {
                            warn!("Skipping match data...");
                            warn!("Data: {} {} {} {} {} {} {} {}", match_date, league_id, team1_id.unwrap_or(-1), team2_id.unwrap_or(-1), stage.clone().unwrap_or("".to_string()), round_name, team1_score.unwrap_or(-1), team2_score.unwrap_or(-1));
                            continue;
                        }
                        
                        let insert_res = conn.execute(
                            "INSERT INTO matches (date, league_id, team1_id, team2_id, stage, round, team1_score, team2_score)
                            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                            params![
                                match_date,
                                league_id,
                                team1_id.unwrap_or(-1),
                                team2_id.unwrap_or(-1),
                                stage,
                                round_name,
                                team1_score,
                                team2_score
                            ]
                        );
                        
                        if let Err(e) = insert_res {
                            warn!("Failed to insert match: {}", e.to_string());
                            warn!("Data: {} {} {} {} {} {} {} {}", match_date, league_id, team1_id.unwrap_or(-1), team2_id.unwrap_or(-1), stage.clone().unwrap_or("".to_string()), round_name, team1_score.unwrap_or(-1), team2_score.unwrap_or(-1));
                            continue;
                        }
                    }
                }
            };
        }

        Ok(())
    }
}