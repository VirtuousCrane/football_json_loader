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
#[serde(default)]
pub struct LeagueMatch {
    pub date: String,
    pub team1: String,
    pub team2: String,
    pub score: LeagueScore,
}

impl Default for LeagueMatch {
    fn default() -> Self {
        LeagueMatch { date: String::new(), team1: String::new(), team2: String::new(), score: LeagueScore::default() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LeagueScore {
    pub ft: Vec<i32>,
}

impl LeagueScore {
    pub fn get_team_1_score(&self) -> Option<&i32> {
        self.ft.get(0)
    }
    
    pub fn get_team_2_score(&self) -> Option<&i32> {
        self.ft.get(1)
    }
}

impl Default for LeagueScore {
    fn default() -> Self {
        LeagueScore { ft: vec![-1, -1] }
    }
}