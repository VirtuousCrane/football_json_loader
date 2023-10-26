use rusqlite::{Connection, Result};

pub struct DatabaseProcessor {
    conn: Option<Connection>,
}

impl DatabaseProcessor {
    pub fn new() -> Self {
        DatabaseProcessor { conn: None }
    }

    pub fn init(&mut self, path: &str) -> Result<()> {
        self.conn = Some(Connection::open(path)?);
        
        self.create_teams_table()?;
        self.create_leagues_table()?;
        self.create_matches_table()?;

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
}