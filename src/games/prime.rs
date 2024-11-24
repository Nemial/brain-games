use rand::Rng;
use std::io;

pub fn start() {
    println!("Welcome to Brain Prime!");
    println!("Answer 'yes' if given number is prime. Otherwise answer 'no'.");

    let num: usize = rand::thread_rng().gen_range(1..512);

    println!("{}", num);

    let answer = match is_prime(num) {
        true => "yes",
        false => "no",
    };

    let mut user_answer = String::new();

    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");

    let user_answer = user_answer.trim().to_lowercase();

    if user_answer != answer {
        println!("You lose!");
        println!("Correct answer was {}", answer);
    } else {
        println!("You win!");
    }
}

fn is_prime(num: usize) -> bool {
    if num < 2 {
        return false;
    }

    for i in 2..num / 2 {
        if num % i == 0 {
            return false;
        }
    }

    true
}
