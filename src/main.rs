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

    let number_of_spies = (num_players as f32 / 3.0).ceil() as u32;
    let number_of_resistance = num_players - number_of_spies;

    println!("\nThere is {} resistance and {} spies.", number_of_resistance, number_of_spies);

    // TODO: Handle each round.
}
