use rand::Rng;

pub fn start() {
    println!("Welcome to Brain Even!");
    println!("Answer 'yes' if the number is even, otherwise answer 'no'.");

    let random_num = rand::thread_rng().gen_range(1..=100);

    println!("{random_num}");

    let mut user_answer = String::new();

    let answer = match is_even(random_num) {
        true => "yes",
        false => "no",
    };

    std::io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");

    if user_answer != answer {
        println!("You lose!");
        println!("Correct answer is {}", answer);
    } else {
        println!("You win!");
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
