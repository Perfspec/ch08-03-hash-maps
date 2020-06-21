use std::collections::HashMap;

fn main() {
	//___________________________________________________________________________
	// play with MultiTeamGame struct
	
	let intial_teams = vec![String::from("Green"), String::from("Red")];
	let initial_scores = vec![11, 22];
	
	let mut game = MultiTeamGame {
        scores: intial_teams.into_iter().zip(initial_scores.into_iter()).collect()
	};
	
	game.write_score(String::from("Blue"), 33);
	game.write_score(String::from("Yellow"), 44);
	
	// will overwrite previous value
	game.write_score(String::from("Blue"), 55);
	
	// will not overwrite previous value
	game.write_score_only_if_empty(String::from("Yellow"), 66);
	
	
	game.get_all_scores();
	
	let team_name_searches = vec![String::from("Blue"), String::from("Orange")];
	
	game.get_score(&team_name_searches[0]);
	game.get_score(&team_name_searches[1]);
	
	//___________________________________________________________________________
	// count how many times each word appears
	
	let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
	
	//___________________________________________________________________________
	// Given a list of integers, use a vector and return the mean (the average
	// value), median (when sorted, the value in the middle position), and mode
	// (the value that occurs most often; a hash map will be helpful here) of the list.
	
	let list_of_integers = vec![1995,1993,1959,1990,1988,1958, 1958];
	
	println!("mean: {}", mean(&list_of_integers));
	println!("median: {}", median(&list_of_integers));
	println!("mode: {}", mode(&list_of_integers));	
}

fn mean(vector: &Vec<usize>) -> f32 {
	let mut sum = 0;
	let mut size = 0;
	for number in vector {
		sum += number;
		size += 1;
	}
	(sum as f32)/(size as f32)
}
fn median(vector: &Vec<usize>) -> usize {
	let mut vectwo = vector.clone();
	vectwo.sort_unstable();
	let len_v = vectwo.len();
	if len_v % 2 == 1 {
		let mid_v = (len_v - 1)/2;
		vectwo[mid_v]
	}
	else {
		let mid_v = len_v/2;
		vectwo[mid_v]
	}
}
fn mode(vector: &Vec<usize>) -> usize {
	let mut map: HashMap<usize, usize> = HashMap::new();
	for &number in vector {
		match map.get(&number) {
			Some(&tally) => map.insert(number, tally + 1),
			None => map.insert(number, 1),
		};
	}
	let mut mode_key =  std::usize::MIN;
	let mut mode_value =  std::usize::MIN;
	for &key in map.keys() {
		match map.get(&key) {
			Some(&tally) => {
				if tally > mode_value {
					mode_value = tally;
					mode_key = key;
				}
			},
			None => (),
		};
	}
	mode_key
}

struct MultiTeamGame {
	scores: HashMap<String, usize>
}

impl MultiTeamGame {
	fn write_score(&mut self, team_name: String, team_score: usize) {
		self.scores.insert(team_name,team_score);
	}
	fn write_score_only_if_empty(&mut self, team_name: String, team_score: usize) {
		self.scores.entry(team_name).or_insert(team_score);
	}
	fn get_all_scores(&self) {
		for (key, value) in &self.scores {
			println!("{}: {}", key, value);
		}
	}
	fn get_score(&self, team_name: &String) {
		match self.scores.get(team_name) {
			Some(value) => println!("The score for {} is {}", team_name, value),
			None => println!("No score found for {}", team_name),
		}
	}
}