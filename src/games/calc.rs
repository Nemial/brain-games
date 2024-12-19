use crate::games::start_game;
use rand::prelude::SliceRandom;
use rand::Rng;

const GAME_NAME: &str = "Brain Calc";
const GAME_DESCRIPTION: &str = "What is the result of the expression?";

pub fn start() {
    let operations: [char; 3] = ['*', '+', '-'];
    let mut rng = rand::thread_rng();

    let first_operand: usize = rng.gen_range(0..512);
    let second_operand: usize = rng.gen_range(0..first_operand);

    let operation = operations
        .choose(&mut rng)
        .expect("Fail return random operation");

    let expression = format!("{} {} {}", first_operand, operation, second_operand);
    let expression_result = get_expression_result(first_operand, second_operand, operation);

    start_game(
        GAME_NAME,
        GAME_DESCRIPTION,
        expression_result.to_string(),
        expression,
    );
}

fn get_expression_result(first_operand: usize, second_operand: usize, operation: &char) -> usize {
    match operation {
        '*' => first_operand * second_operand,
        '+' => first_operand + second_operand,
        '-' => first_operand - second_operand,
        _ => panic!("Unrecognized operation {}", operation),
    }
}

#[test]
fn check_expression_result() {
    assert_eq!(get_expression_result(2, 2, &'*'), 4);
    assert_eq!(get_expression_result(2, 2, &'-'), 0);
    assert_eq!(get_expression_result(2, 5, &'+'), 7);
}

#[test]
#[should_panic]
fn check_panic_result() {
    get_expression_result(2, 2, &'A');
}
