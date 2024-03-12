use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use std::process::id;
use std::sync::Mutex;
use super::team::Team;
static mut NEXT_PLAYER_ID : Mutex<u32> = Mutex::new(0);

// Skills have a Value from 1-255
#[derive(Debug)]
pub struct Skills {
    pub pace : u8,
    pub power : u8,
    pub technique : u8,
    pub stamina : u8,
    pub vision : u8,    
}

/// Represents a Player in the League
#[derive(Debug)]
pub struct Player {
    pub name : String,
    pub first_name : String,
    pub age : u8,
    pub number : u8,
    pub skills : Skills,
    id : u32,
    pub team : Option<Team>
}

impl Player {
    /// Creates a new Player using passed parameters. 
    pub fn new_player(name: String, first_name: String, age: u8, number: u8, skills: Skills, team : Option<Team>) -> Player {
        let mut id_m = unsafe {NEXT_PLAYER_ID.lock().unwrap()};
        let id = *id_m;
        
        println!("Creating new player with ID: {id}");
        *id_m += 1;
        println!("Next Player ID will be: {id_m}");
        Player{
            name, first_name, age, number, skills, id, team
        }

    }

    /// Generates a random player, with an optional skillcap and skillfloor.
    /// skillcap has a higher priority than skillfloor.
    pub fn gen_player(skillcap : Option<u8>, skillfloor : Option<u8>) -> Player {
        let mut id_m = unsafe {NEXT_PLAYER_ID.lock().expect("Whoops")};
        let id = *id_m;
        println!("Creating new randomly generated player with ID: {id}");
        *id_m += 1;
        println!("Next Player ID will be: {id_m}");
        drop(id_m);
        let skills = gen_random_skills(skillcap, skillfloor);
        let name = "Doe".to_string();
        let first_name = "John".to_string();
        let mut small_rng = SmallRng::from_entropy();
        let age: u8 = small_rng.gen_range(17..50);
        let number : u8 = small_rng.gen_range(0..100);

        Player{ name, first_name, age, number, skills, id, team: None}
    }

}

/// Create a random set of Skills ranging from 1-255.
/// - `skillcap` limits the maximum generated skill level.
/// - `skillfloor` places a minimum generated skill level.
/// 
/// If `skillcap < skillfloor`, all values will become the skillcap. 
fn gen_random_skills(skillcap : Option<u8>, skillfloor : Option<u8>) -> Skills {
    let skillcap : u8 = skillcap.unwrap_or(255);
    let skillfloor : u8 = skillfloor.unwrap_or(1).min(skillcap);
    let mut small_rng = SmallRng::from_entropy();
    Skills { pace:  small_rng.gen_range(skillfloor..=skillcap) as u8,
         power:     small_rng.gen_range(skillfloor..=skillcap) as u8, 
         technique: small_rng.gen_range(skillfloor..=skillcap) as u8, 
         stamina:   small_rng.gen_range(skillfloor..=skillcap) as u8, 
         vision:    small_rng.gen_range(skillfloor..=skillcap) as u8
        }
}


