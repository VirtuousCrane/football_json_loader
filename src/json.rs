use std::{path::{PathBuf, Path}, fs, io, collections::HashMap, rc::Rc};

use log::{warn, info};

use crate::model::{Team, MatchTeamList, LeagueJsonFormat};

pub struct JsonLoader {
    files: Vec<PathBuf>,
    pub teams: Vec<Rc<Team>>,
    team_name_map: HashMap<String, Rc<Team>>,
    pub leagues: Vec<LeagueJsonFormat>,
    is_initialized: bool
}

impl JsonLoader {
    pub fn new() -> Self {
        return JsonLoader {
            files: Vec::new(),
            teams: Vec::new(),
            team_name_map: HashMap::new(),
            leagues: Vec::new(),
            is_initialized: false,
        };
    }
    
    pub fn init(&mut self) -> Result<(), io::Error> {
        let mut root_path_list: Vec<PathBuf> = Vec::new();
        
        let entries = fs::read_dir(Path::new("/tmp/football.json"))?;
        for entry in entries {
            let dir_entry = entry?;
            let dir_entry_file_type = dir_entry.file_type()?;
            
            if !dir_entry_file_type.is_dir() || dir_entry.path().ends_with(".git") {
                continue;
            }
            
            let entry_path = dir_entry.path();
            info!("Found Directory: {:?}", entry_path);
            root_path_list.push(entry_path);
        }
        
        self.load_files(&root_path_list);
        self.is_initialized = true;
        Ok(())
    }
    
    /// Stores all file paths found in all sub-directories into an internal vector
    fn load_files(&mut self, path_list: &Vec<PathBuf>) -> &Vec<PathBuf> {
        for path in path_list {
            match self.explore_dir(path) {
                Ok(mut directory_file_list) => self.files.append(&mut directory_file_list),
                Err(e) => {
                    warn!("Failed to explore: {:?} because: {}", path, e.to_string());
                    continue;
                }
            }
        }
        
        return &self.files;
    }

    /// Loads all teams from all files. Will return None if struct has not been initialized.
    pub fn load_teams(&mut self) -> Option<&Vec<Rc<Team>>> {
        if !self.is_initialized {
            warn!("JsonLoader not yet initialized. This function call will return nothing");
            return None;
        }

        for file_path in &self.files {
            if !file_path.to_string_lossy().contains("clubs") {
                continue;
            }

            let file_content = match fs::read_to_string(file_path) {
                Ok(f) => f,
                Err(e) => {
                    warn!("Failed to read: {} because: {}", file_path.to_string_lossy(), e.to_string());
                    continue;
                }
            };
            
            let match_team_list: MatchTeamList = match serde_json::from_str(&file_content) {
                Ok(match_team) => match_team,
                Err(e) => {
                    warn!("Failed to deserialize: {} because: {}", file_path.to_string_lossy(), e.to_string());
                    continue;
                }
            };
            
            for mut team in match_team_list.teams {
                let mut found = false;
                for t in self.teams.iter() {
                    if t.name.eq(&team.name) {
                        found = true;
                    }
                }
                
                if found {
                    continue;
                }
                
                team.id = self.teams.len() as i32 + 1;
                
                let team_rc = Rc::new(team);
                self.teams.push(team_rc.clone());
                self.team_name_map.insert(team_rc.name.clone(), team_rc.clone());
            }
        }
        
        return Some(self.get_teams());
    }
    
    pub fn load_leagues(&mut self) -> Option<&Vec<LeagueJsonFormat>> {
        if !self.is_initialized {
            warn!("JsonLoader not yet initialized. This function call will return nothing");
            return None;
        }
        
        for file_path in &self.files {
            if file_path.to_string_lossy().contains("clubs") || file_path.to_string_lossy().contains("groups") {
                continue;
            }
            
            let file_content = match fs::read_to_string(file_path) {
                Ok(f) => f,
                Err(e) => {
                    warn!("Failed to read: {} because: {}", file_path.to_string_lossy(), e.to_string());
                    continue;
                }
            };
            
            let mut league: LeagueJsonFormat = match serde_json::from_str(&file_content) {
                Ok(l) => l,
                Err(e) => {
                    warn!("Failed to deserialize: {} because: {}", file_path.to_string_lossy(), e.to_string());
                    continue;
                }
            };

            league.set_league_id(self.leagues.len() as i32 + 1);
            self.leagues.push(league);
        }
        
        Some(&self.leagues)
    }

    /// Reads all items in a directory
    fn explore_dir(&self, path: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
        info!("Exploring: {}", path.to_str().unwrap_or_default());
        let mut file_path_list: Vec<PathBuf> = Vec::new();
        let dir_items = fs::read_dir(path)?;

        for entry in dir_items {
            file_path_list.push(entry?.path());
        }
        
        return Ok(file_path_list);
    }
    
    pub fn get_teams(&self) -> &Vec<Rc<Team>> {
        &self.teams
    }
    
    pub fn get_leagues(&self) -> &Vec<LeagueJsonFormat> {
        &self.leagues
    }
    
    pub fn get_team_id_from_name(&self, name: &String) -> Option<i32> {
        match self.team_name_map.get(name) {
            Some(t) => Some(t.id),
            None => None
        }
    }
}