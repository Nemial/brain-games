use clap::ValueEnum;

pub mod calc;
pub mod even;
pub mod gcd;
pub mod prime;
pub mod progression;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Games {
    /// Brain Calc
    Calc,
    /// Brain Even
    Even,
    /// Brain Gcd
    Gcd,
    /// Brain Prime
    Prime,
    /// Brain Progression
    Progression,
}

fn start_game(name: &str, description: &str, answer: &str, question: &str) {
    println!("Welcome to the {name}");
    println!("{description}");

    println!("{question}");

    let mut user_answer = String::new();
    std::io::stdin().read_line(&mut user_answer).unwrap();

    user_answer = user_answer.trim().to_lowercase();

    if user_answer == answer {
        println!("You win!");
    } else {
        println!("You lose!");
        println!("Correct answer: {answer}");
    }
}
