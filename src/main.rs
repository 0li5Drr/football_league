pub mod lib {
    pub mod league;
    pub mod player;
    pub mod team;
}

use lib::player::Player;
fn main() {
    for _ in 0..3 {
        let p = Player::gen_player(Some(50), Some(70));
        println!("{p:#?}");
    }
}
