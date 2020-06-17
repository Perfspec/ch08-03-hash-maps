fn main() {
    use std::collections::HashMap;
	
	let teams = vec![String::from("Green"), String::from("Red")];
    let initial_scores = vec![11, 22];
	
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    scores.insert(String::from("Blue"), 33);
    scores.insert(String::from("Yellow"), 44);
	
	println!("{:#?}", scores);
}
