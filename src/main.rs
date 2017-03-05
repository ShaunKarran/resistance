use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

extern crate serde_yaml;

fn main() {
    // -------- File Input --------
    // TODO: better way to store round information. Current way is so bad.
    let file_name = "game/rounds.yaml";
    let mut file = File::open(file_name)
        .expect("File could not be opened");

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let round_information: HashMap<u32, HashMap<u32, HashMap<String, u32>>> = serde_yaml::from_str(&data).unwrap();
    // Debugging stuff
    // let round_number = 1;
    // println!("{}", round_information[&number_of_players][&round_number][&"num_on_mission".to_string()]);

    // -------- Game Setup --------
    println!("Welcome to The Resistance game solver.");
    print!("\nEnter the number of players: ");
    io::stdout().flush() // This is needed because print! is line buffered.
        .unwrap();

    // TODO: Just get number of players as command line argument.
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let number_of_players: u32 = input.trim().parse() // Trim the new line from the end of the input.
        .expect("Failed to parse input, input a positive integer");

    // TODO: Figure out what needs to be stored about a player, and change the data type to suit.
    let mut players = HashMap::new();
    for x in 1..(number_of_players + 1) {
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

    let number_of_spies = (number_of_players as f32 / 3.0).ceil() as u32;
    let number_of_resistance = number_of_players - number_of_spies;

    println!("There is {} resistance and {} spies.", number_of_resistance, number_of_spies);

    // TODO: Handle each round.
}
