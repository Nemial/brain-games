use brain_games::games::{calc, even, gcd, prime, progression};
use std::io;

fn main() {
    println!("Welcome to the Brain Games!");
    println!("Please choose the game");
    println!("even, gcd, prime, progression, calc");

    let mut game_name = String::new();

    io::stdin()
        .read_line(&mut game_name)
        .expect("Failed to read line");

    match game_name.trim().to_lowercase().as_str() {
        "even" => even::start(),
        "calc" => calc::start(),
        "gcd" => gcd::start(),
        "prime" => prime::start(),
        "progression" => progression::start(),
        _ => eprintln!("Not a valid game name"),
    }
}
