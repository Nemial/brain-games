use crate::games::start_game;
use rand::Rng;

const GAME_NAME: &str = "Brain Even";
const GAME_DESCRIPTION: &str = "Answer 'yes' if the number is even, otherwise answer 'no'.";
pub fn start() {
    let random_num = rand::thread_rng().gen_range(1..=100);

    let answer = if is_even(random_num) { "yes" } else { "no" };

    start_game(GAME_NAME, GAME_DESCRIPTION, answer, &random_num.to_string());
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[test]
fn check_is_even() {
    assert!(is_even(10));
    assert!(!is_even(9));
}
