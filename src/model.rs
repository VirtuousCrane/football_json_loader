use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MatchTeamList {
    pub name: String,
    #[serde(rename="clubs")]
    pub teams: Vec<Team>
}

#[derive(Serialize, Deserialize)]
#[derive(Eq, Hash, Clone)]
pub struct Team {
    #[serde(skip_deserializing, default)]
    pub id: i32,
    pub name: String,
    pub code: Option<String>
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}