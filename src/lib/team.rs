use super::player::Player;

/// Represents a Team in the League
#[derive(Debug, Clone)]
pub struct Team {
    pub name : String,
    pub players : Vec<Player>,
    pub points : u8,
    pub goal_difference : u8,
}

impl Team {
    pub fn generate_team() -> Team {
        todo!()
    }

    /// Generates a teamname.
    pub fn gen_name() -> String {
        todo!()
    }
}