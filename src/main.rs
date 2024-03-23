use std::io; 

fn main() {
    let candidates = vec![
        Candidate::new("Candidate A", "Party W"),
        Candidate::new("Candidate B", "Party X"),
        Candidate::new("Candidate C", "Party Y"),
        Candidate::new("Candidate D", "Party Z"),
    ];

    println!("Welcome to the Electronic Voting Machine!");
    println!("Candidates:");
    for (i, candidate) in candidates.iter().enumerate() {
        println!("{}. {} ({})", i + 1, candidate.name, candidate.party);
    }

    println!("Please enter the number of your chosen candidate:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let choice: usize = input.trim().parse().expect("Please enter a number");

    if choice > 0 && choice <= candidates.len() {
        let chosen_candidate = &candidates[choice - 1];
        println!(
            "You have voted for {} from {}.",
            chosen_candidate.name, chosen_candidate.party
        );
    } else {
        println!("Invalid candidate number. Please try again.");
    }
}
#[derive(Debug)]
struct Candidate {
    name: String,
    party: String,
}

impl Candidate {
    fn new(name: &str, party: &str) -> Self {
        Candidate {
            name: name.to_string(),
            party: party.to_string(),
        }
    }
}
