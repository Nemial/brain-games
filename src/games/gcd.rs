use rand::Rng;
use std::io;

pub fn start() {
    println!("Welcome to Brain GCD!");
    println!("Find the greatest common divisor of given numbers.");

    let mut rng = rand::thread_rng();

    let first_num: usize = rng.gen_range(1..256);
    let second_num: usize = rng.gen_range(1..first_num);

    let question = format!("{} {}", first_num, second_num);
    let answer = find_gcd(first_num, second_num);

    let mut user_answer = String::new();

    println!("{}", question);

    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");

    let user_answer = user_answer
        .trim()
        .parse::<usize>()
        .expect("Please type a number!");

    if user_answer != answer {
        println!("You lose!");
        println!("Correct answer was {}", answer);
    } else {
        println!("You win!");
    }
}

fn find_gcd(first_num: usize, second_num: usize) -> usize {
    if first_num % second_num != 0 {
        find_gcd(second_num, first_num % second_num)
    } else {
        second_num
    }
}
