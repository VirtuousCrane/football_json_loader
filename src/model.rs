use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MatchTeamList {
    pub name: String,
    #[serde(rename="clubs")]
    pub teams: Vec<Team>
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Team {
    #[serde(skip_deserializing, default)]
    pub id: i32,
    pub name: String,
    pub code: Option<String>
}


#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeagueJsonFormat {
    OldFormat(League),
    NewFormat(NewLeague)
}

pub trait LeagueMatchObject {
    fn get_date(&self) -> &str;
    fn get_team_1_name(&self) -> &str;
    fn get_team_2_name(&self) -> &str;
    fn get_team_1_score(&self) -> Option<i32>;
    fn get_team_2_score(&self) -> Option<i32>;
}

#[derive(Serialize, Deserialize)]
pub struct League {
    pub name: String,
    pub rounds: Vec<LeagueRound>,
}

#[derive(Serialize, Deserialize)]
pub struct LeagueRound {
    pub name: String,
    pub matches: Vec<LeagueMatch>,
}

#[derive(Serialize, Deserialize)]
pub struct LeagueMatch {
    pub date: String,
    pub team1: String,
    pub team2: String,
    pub score: Option<LeagueScoreFormat>,
}

impl Default for LeagueMatch {
    fn default() -> Self {
        LeagueMatch { date: String::new(), team1: String::new(), team2: String::new(), score: None }
    }
}

impl LeagueMatchObject for LeagueMatch {
    fn get_date(&self) -> &str {
        &self.date
    }

    fn get_team_1_name(&self) -> &str {
        &self.team1
    }

    fn get_team_2_name(&self) -> &str {
        &self.team2
    }

    fn get_team_1_score(&self) -> Option<i32> {
        if let Some(s) = &self.score {
            return Some(s.get_team_1_score());
        }
        
        None
    }

    fn get_team_2_score(&self) -> Option<i32> {
        if let Some(s) = &self.score {
            return Some(s.get_team_2_score());
        }
        
        None
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeagueScoreFormat {
    Normal(NormalLeagueScore),
    Australian(AustralianScore),
}

pub trait LeagueScore {
    fn get_team_1_score(&self) -> i32;
    fn get_team_2_score(&self) -> i32;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NormalLeagueScore {
    pub ft: Vec<i32>,
}

impl LeagueScore for NormalLeagueScore {
    fn get_team_1_score(&self) -> i32 {
        self.ft[0]
    }
    
    fn get_team_2_score(&self) -> i32 {
        self.ft[1]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AustralianScore {
    pub et: Vec<i32>,
    pub p: Vec<i32>,
}

impl LeagueScore for AustralianScore {
    fn get_team_1_score(&self) -> i32 {
        self.et[0]
    }
    
    fn get_team_2_score(&self) -> i32 {
        self.et[1]
    }
}

impl LeagueScore for LeagueScoreFormat {
    fn get_team_1_score(&self) -> i32 {
        match self {
            LeagueScoreFormat::Normal(n) => n.get_team_1_score(),
            LeagueScoreFormat::Australian(a) => a.get_team_2_score(),
        }
    }
    fn get_team_2_score(&self) -> i32 {
        match self {
            LeagueScoreFormat::Normal(n) => n.get_team_2_score(),
            LeagueScoreFormat::Australian(a) => a.get_team_2_score(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewLeague {
    pub name: String,
    pub matches: Vec<NewLeagueMatch>
}

#[derive(Serialize, Deserialize)]
pub struct NewLeagueMatch {
    stage: Option<String>,
    round: String,
    #[serde(flatten)]
    match_info: LeagueMatch,
}

impl LeagueMatchObject for NewLeagueMatch {
    fn get_date(&self) -> &str {
        self.match_info.get_date()
    }

    fn get_team_1_name(&self) -> &str {
        self.match_info.get_team_1_name()
    }

    fn get_team_2_name(&self) -> &str {
        self.match_info.get_team_2_name()
    }

    fn get_team_1_score(&self) -> Option<i32> {
        self.match_info.get_team_1_score()
    }

    fn get_team_2_score(&self) -> Option<i32> {
        self.match_info.get_team_2_score()
    }
}