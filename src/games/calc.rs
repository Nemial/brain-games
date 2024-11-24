use rand::prelude::SliceRandom;
use rand::Rng;
use std::io;

pub fn start() {
    println!("Brain Calc");
    println!("What is the result of the expression?");

    let operations: [char; 3] = ['*', '+', '-'];
    let mut rng = rand::thread_rng();

    let first_operand: usize = rng.gen_range(0..512);
    let second_operand: usize = rng.gen_range(0..first_operand);

    let operation = operations.choose(&mut rng).unwrap();

    let expression = format!("{} {} {}", first_operand, operation, second_operand);
    let expression_result = get_expression_result(first_operand, second_operand, operation);

    println!("{}", expression);

    let mut user_answer = String::new();
    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");

    let user_answer = user_answer
        .trim()
        .parse::<usize>()
        .expect("Failed to convert user_answer");

    if user_answer != expression_result {
        println!("You lost!");
        println!("Correct answer was: {}", expression_result);
    } else {
        println!("You won!");
    }
}

fn get_expression_result(first_operand: usize, second_operand: usize, operation: &char) -> usize {
    match operation {
        '*' => first_operand * second_operand,
        '+' => first_operand + second_operand,
        '-' => first_operand - second_operand,
        _ => panic!("Unrecognized operation {}", operation),
    }
}
