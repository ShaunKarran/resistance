#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
// extern crate serde_yaml;

use std::collections::HashMap;
// use std::fs::File;
use std::io::{self, Write};

use clap::App;

#[derive(Debug, Serialize, Deserialize)]
struct RoundData {
    attempts: Vec<AttemptData>,
    mission_success: bool,
}

impl RoundData {
    fn new() -> RoundData {
        RoundData { attempts: Vec::new(), mission_success: false }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AttemptData {
    players_on_mission: Vec<String>,
    votes: HashMap<String, bool>,
}

impl AttemptData {
    fn new() -> AttemptData {
        AttemptData { players_on_mission: Vec::new(), votes: HashMap::new() }
    }
}

const NUM_MISSIONS: usize = 5;

// The inner arrays start with 4 0's because they are indexed on the number of players in the game, min 5.
// Example of usage: num_players_on_mission = round_info[mission_number][num_players_in_game]
const NUM_PLAYERS_PER_MISSION: [[u32; 10]; NUM_MISSIONS] = [
    /* Mission 1 */ [0, 0, 0, 0, 2, 2, 2, 3, 3, 3],
    /* Mission 2 */ [0, 0, 0, 0, 3, 3, 3, 4, 4, 4],
    /* Mission 3 */ [0, 0, 0, 0, 2, 4, 3, 4, 4, 4],
    /* Mission 4 */ [0, 0, 0, 0, 3, 3, 4, 5, 5, 5],
    /* Mission 5 */ [0, 0, 0, 0, 3, 4, 4, 5, 5, 5],
];

// The number of fails required for the spys to win a given mission.
const NUM_FALS_PER_MISSION: [[u32; 10]; NUM_MISSIONS] = [
    /* Mission 1 */ [0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
    /* Mission 2 */ [0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
    /* Mission 3 */ [0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
    /* Mission 4 */ [0, 0, 0, 0, 1, 1, 2, 2, 2, 2],
    /* Mission 5 */ [0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
];

fn main() {
    let yaml = load_yaml!("cli.yaml"); // NOTE: Currently clap yaml feature is incompatible with serde_yaml.
    let matches = App::from_yaml(yaml).get_matches();

    // Parse the cmd line argument as a u32.
    let num_players: u32 = matches.value_of("num-players").unwrap().parse()
        .expect("Failed to parse input, input a positive integer");

    // -------- Game Setup --------
    println!("Welcome to The Resistance game solver.");

    // Record the names of the players in the game.
    let mut player_names = Vec::new();
    for x in 0..num_players {
        print!("\nEnter the name of player {}: ", x + 1); // + 1 so that it begins at 1.
        io::stdout().flush().unwrap(); // This is needed because print! is line buffered.

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let player_name = input.trim().to_string(); // Trim the new line.

        player_names.push(player_name);
    }

    // TODO: Bonus cards. (Commander, Assassin, etc.)

    // Calculate team sizes.
    let num_spies = (num_players as f32 / 3.0).ceil() as u32;
    let num_resistance = num_players - num_spies;
    println!("\nThere is {} resistance members and {} spies.", num_resistance, num_spies);

    // Object to store the data recorded during the game.
    let mut game_data: Vec<RoundData> = Vec::new();

    // -------- Game Rounds --------
    'mission: for mission_number in 0..NUM_MISSIONS {
        println!("\nStarting mission {}.", mission_number + 1); // + 1 so that it begins at 1.

        // Determine the number of players going on the mission.
        let num_players_on_mission = NUM_PLAYERS_PER_MISSION[mission_number][num_players as usize];

        // Object to store the data recorded during the round.
        let mut round_data = RoundData::new();

        'attempt: for mission_attempt in 0..6 {
            println!("\nAttempt number {}.", mission_attempt + 1); // + 1 so that it begins at 1.

            // Object to store the data recorded during the mission attempt.
            let mut attempt_data = AttemptData::new();

            // Record the names of the players going on the mission.
            for _ in 0..num_players_on_mission {
                print!("\nEnter the name of a player going on the mission: ");
                io::stdout().flush().unwrap(); // This is needed because print! is line buffered.

                // Read the input from the user.
                let mut input = String::new();
                io::stdin().read_line(&mut input)
                    .expect("Failed to read line");
                let player_name = input.trim().to_string(); // Trim the new line.

                // Store the players name in the attempt data.
                attempt_data.players_on_mission.push(player_name);
            }

            // Record the votes of all the players.
            for player_name in player_names.iter() {
                print!("\nEnter {}'s vote (a[ccept] or r[eject]): ", player_name);
                io::stdout().flush().unwrap(); // This is needed because print! is line buffered.

                // Read the input from the user.
                let mut input = String::new();
                io::stdin().read_line(&mut input)
                    .expect("Failed to read line");
                let vote = input.trim().to_string(); // Trim the new line.

                // Store the vote in the attempt data.
                let player_name = player_name.clone(); // Clone the player name for insertion into hashmap.
                if vote == "a" || vote == "accept" {
                    attempt_data.votes.insert(player_name, true);
                } else if vote == "r" || vote == "reject" {
                    attempt_data.votes.insert(player_name, false);
                }
            }

            round_data.attempts.push(attempt_data);

            if vote_passed(&round_data.attempts.last().unwrap().votes) {
                println!("Players go on the mission.");
                // Record the number of fail cards from the mission.
                // TODO last: Do fancy calculation to work out probability of spies.

                round_data.mission_success = true;

                println!("{:?}", round_data);
                game_data.push(round_data);

                // If the vote passes dont attempt mission again. Break to the start of the outer loop.
                continue 'mission;
            }
        }

        // If it gets to here, too many missions failed, spys win the round.
        println!("Too many failed attempts. Spies win.");
    }
}

// TODO: How to write proper docstring.
/// Returns true if more than half the votes are yes, otherwise false.
fn vote_passed(votes: &HashMap<String, bool>) -> bool {
    let mut num_accepts = 0;
    let num_players = votes.len();

    for (_, vote) in votes.iter() {
        if *vote {
            num_accepts += 1;
        }

        // Vote passes as soon as more than half the players accept.
        if num_accepts > (num_players / 2) {
            return true;
        }
    }

    // It not enough votes.
    return false;
}
