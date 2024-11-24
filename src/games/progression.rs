use rand::Rng;
use std::collections::BTreeMap;
use std::io;

pub fn start() {
    println!("Welcome to Brain Progression!");
    println!("What number is missing in the progression?");

    let mut rng = rand::thread_rng();
    let progression_length: usize = 10;
    let start_num: usize = rng.gen_range(0..512);
    let step: usize = rng.gen_range(1..5);

    let mut progression = gen_progression(start_num, progression_length, step);
    let hidden_key = rng.gen_range(0..progression_length);
    let hidden_mask = String::from("...");

    let answer = progression
        .get(&hidden_key)
        .expect("Hidden key not found")
        .to_owned();

    progression.insert(hidden_key, hidden_mask);

    let mut question = String::new();

    for (_, value) in progression.iter() {
        question.push_str(value);
        question.push_str(" ");
    }

    println!("{}", question);

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

fn gen_progression(
    start_num: usize,
    progression_length: usize,
    step: usize,
) -> BTreeMap<usize, String> {
    let mut progression: BTreeMap<usize, String> = BTreeMap::new();
    let mut i = 0;

    while progression.len() < progression_length {
        let progression_item = start_num + i * step;
        progression.insert(i, progression_item.to_string());

        i += 1;
    }

    progression
}
