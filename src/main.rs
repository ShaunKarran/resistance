#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
// extern crate serde_yaml;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

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

const NUM_MISSIONS: u32 = 5;

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

    let num_players: u32 = matches.value_of("num-players").unwrap().parse()  // Parse the string to u32.
        .expect("Failed to parse input, input a positive integer");

    // -------- File Input --------
    // TODO: better way to store round information. Current way is so bad.
    // let file_name = "game/rounds.yaml";
    // let mut file = File::open(file_name)
    //     .expect("File could not be opened");

    // let mut data = String::new();
    // file.read_to_string(&mut data).unwrap();

    // let round_information: HashMap<u32, HashMap<u32, HashMap<String, u32>>> = serde_yaml::from_str(&data).unwrap();
    // Debugging stuff
    // let mission_number = 1;
    // println!("{}", round_information[&num_players][&mission_number][&"num_on_mission".to_string()]);

    // -------- Game Setup --------
    println!("Welcome to The Resistance game solver.");

    let mut players = Vec::new();
    for x in 1..(num_players + 1) {
        print!("\nEnter the name of player {}: ", x);
        io::stdout().flush() // This is needed because print! is line buffered.
            .unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let player_name = input.trim().to_string();

        players.push(player_name);
    }

    // TODO: Bonus cards. (Commander, Assassin, etc.)

    let num_spies = (num_players as f32 / 3.0).ceil() as u32;
    let num_resistance = num_players - num_spies;

    println!("\nThere is {} resistance members and {} spies.", num_resistance, num_spies);

    // Object to store all the data recorded during the game.
    let mut game_data: Vec<RoundData> = Vec::new();

    // -------- Game Rounds --------
    'mission: for mission_number in 0..NUM_MISSIONS {
        println!("\nStarting mission {}.", mission_number + 1); // + 1 so that it begins at 1.

        let mut round_data = RoundData::new();
        let num_players_on_mission = 2; // TODO: This will come from the round information.

        'attempt: for mission_attempt in 0..6 {
            let mut attempt_data = AttemptData::new();

            // Record the names of the players going on the mission.
            // let mut players_on_mission = Vec::new();
            for x in 0..num_players_on_mission {
                print!("\nEnter the name of a player going on the mission: ");
                io::stdout().flush() // This is needed because print! is line buffered.
                    .unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input)
                    .expect("Failed to read line");
                let player_name = input.trim().to_string(); // Trim the new line.

                attempt_data.players_on_mission.push(player_name);
            }

            // Record the votes of all the players.
            let vote_result = "Record the votes.";
            attempt_data.votes.insert("test".to_string(), true);

            round_data.attempts.push(attempt_data);

            // If the vote passes dont attempt mission again.
            if vote_passed() { // vote_passed(vote_result, num_players);
                println!("Players go on the mission.");
                // Record the number of fail cards from the mission.
                // TODO last: Do fancy calculation to work out probability of spies.

                round_data.mission_success = true;
                println!("{:?}", round_data);
                game_data.push(round_data);
                continue 'mission; // Break to the start of the outer loop.
            }
        }

        // If it gets to here, too many missions failed, spys win the round.
        println!("Too many failed attempts. Spies win.");
    }
}

fn vote_passed() -> bool {
    return true;
}
