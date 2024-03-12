use crate::lib::team::Team;

pub trait FootballLeague {
    /// Simulates a game. 
    /// Takes 2 teams, and returns the winner, and returns the score.
    /// 
    /// Also updates the teams' Goal Difference and points. (3 for win, 1 for draw, 0 for loss) 
    fn simulate_game(team1 : &mut Team, team2 : &mut Team) -> (u8,u8);

    /// Simulates entire League
    /// 
    /// Will simulate games until the tournament is over. After each round of games (i.e. Game Day), will print an updated Leaderboard/Bracket
    fn simulate_league(&mut self) -> ();

    /// Initialises the League
    /// 
    /// Randomly generates the Teams and Players belonging to them.
    fn initialise_leauge() -> Self;
    /// Initialises the League using Teams passed.
    /// 
    /// If too few Teams exist, the remaining Teams will be generated randomly. If too many Teams are given, the extra teams will be ignored.
    fn initialise_league_w_teams(teams : Vec<Team>) -> Self;
}



/// Represents a 3 Stage Tournament (8 Teams)
/// Stage 1 & 2 are Best of 3, Stage 3 is one match.
pub struct ThreeStageTournament {
    teams : Vec<Team>,

}

impl FootballLeague for ThreeStageTournament {
    fn simulate_game(team1 : &mut Team, team2 : &mut Team) -> (u8,u8) {
        todo!()
    }

    fn initialise_leauge() -> Self {
        let mut league = ThreeStageTournament { teams: vec!()};
        for _ in 0..8 {
            let mut team = Team::generate_team();
            // Ensure all Teams have unique names.
            'check_name: loop  { 
                for t in league.teams.iter() {
                    if t.name.eq(&team.name) {
                        team.name= Team::gen_name();
                        continue 'check_name;
                    }
                }
                break;
            }
            league.teams.push(team);
            
        }
        return league;
    }

    fn initialise_league_w_teams(teams : Vec<Team>) -> Self {
        let mut teams = teams;
         
        if teams.len() > 8 {
            teams =  teams.split_at(8).0.to_vec();
        } 
        else if teams.len() < 8 {
            for _ in 0..(8-teams.len()) {
                let mut team = Team::generate_team();
            // Ensure all Teams have unique names.
                'check_name: loop  { 
                    for t in teams.iter() {
                        if t.name.eq(&team.name) {
                            team.name= Team::gen_name();
                            continue 'check_name;
                        }
                    }
                    break;
                }
                teams.push(team);
            }
        }
        return ThreeStageTournament{teams}
    }
    
    fn simulate_league(&mut self) -> () {
        todo!()
    }
}