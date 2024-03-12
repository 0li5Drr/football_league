use super::player::Player;

/// Represents a Team in the League
#[derive(Debug)]
pub struct Team {
    players : Vec<Player>,
}