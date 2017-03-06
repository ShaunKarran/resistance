#[macro_use]
extern crate clap;
// extern crate serde_yaml;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

use clap::App;

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
    // let round_number = 1;
    // println!("{}", round_information[&num_players][&round_number][&"num_on_mission".to_string()]);

    // -------- Game Setup --------
    println!("Welcome to The Resistance game solver.");

    // TODO: Figure out what needs to be stored about a player, and change the data type to suit.
    let mut players = HashMap::new();
    for x in 1..(num_players + 1) {
        print!("\nEnter the name of player {}: ", x);
        io::stdout().flush() // This is needed because print! is line buffered.
            .unwrap();

        let mut player_name = String::new();
        io::stdin().read_line(&mut player_name)
            .expect("Failed to read line");

        players.insert(player_name, x);
    }
    // Debugging stuff.
    // println!("The players are");
    // for (name, number) in players.iter() {
    //     println!("Player {}: {}", number, name);
    // }

    // TODO: Bonus cards. (Commander, Assassin, etc.)

    let num_spies = (num_players as f32 / 3.0).ceil() as u32;
    let num_resistance = num_players - num_spies;

    println!("\nThere is {} resistance and {} spies.", num_resistance, num_spies);

    // TODO: Handle each round.

    // -------- Game Rounds --------
    for round_number in 1..6 {
        println!("\nStarting round {}.", round_number);

        let mut vote_fails = true;
        let mut num_fails = 0;
        while vote_fails { // IDEA: Instead of this while loop, use for loop with number of fails, break if vote passes.
            num_fails += 1;
            if num_fails > 5 { // TODO: Should this be 5 or 6?
                // Go to next round.
            }

            // Record the names of the players going on the mission.
            let num_players_on_mission = 2; // TODO: This will come from the round information.
            let mut players_on_mission = Vec::new();
            for x in 0..num_players_on_mission {
                print!("\nEnter the name of a player going on the mission: ");
                io::stdout().flush() // This is needed because print! is line buffered.
                    .unwrap();

                let mut player_name = String::new();
                io::stdin().read_line(&mut player_name)
                    .expect("Failed to read line");

                players_on_mission.push(player_name);
            }

            // Record the votes of all the players.
            let vote_result = "Record the votes.";
            // If the vote fails, go back to recording the player names.
            vote_fails = parse_vote(); // parse_vote(vote_result);
        }

        // Record the number of fail cards from the mission.

        // TODO last: Do fancy calculation to work out probability of spies.
    }
}

fn parse_vote() -> bool {
    return false;
}
