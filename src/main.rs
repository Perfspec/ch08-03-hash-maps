use std::collections::HashMap;

fn main() {
	
	let intial_teams = vec![String::from("Green"), String::from("Red")];
    let initial_scores = vec![11, 22];
	
    let mut game = MultiTeamGame {
        scores: intial_teams.into_iter().zip(initial_scores.into_iter()).collect()
	};
	
	game.create_score(String::from("Blue"), 33);
    game.create_score(String::from("Yellow"), 44);
	
	game.get_all_scores();
	
	let team_name_searches = vec![String::from("Blue"), String::from("Orange")];
	
	game.get_score(&team_name_searches[0]);
	game.get_score(&team_name_searches[1]);
}

struct MultiTeamGame {
	scores: HashMap<String, usize>
}

impl MultiTeamGame {
	fn create_score(&mut self, team_name: String, team_score: usize) {
		self.scores.insert(team_name,team_score);
	}
	fn get_all_scores(&self) {
		println!("scores: {:#?}", self.scores);
	}
	fn get_score(&self, team_name: &String) {
		match self.scores.get(team_name) {
			Some(value) => println!("The score for {} is {}", team_name, value),
			None => println!("No score found for {}", team_name),
		}
	}
}