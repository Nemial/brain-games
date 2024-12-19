use crate::games::start_game;
use rand::Rng;

const GAME_NAME: &str = "Brain Prime";
const GAME_DESCRIPTION: &str = "Answer 'yes' if given number is prime. Otherwise answer 'no'.";

pub fn start() {
    let num: usize = rand::thread_rng().gen_range(1..512);

    let answer = match is_prime(num) {
        true => "yes",
        false => "no",
    };

    start_game(
        GAME_NAME,
        GAME_DESCRIPTION,
        answer.to_string(),
        num.to_string(),
    );
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

#[test]
fn check_is_prime() {
    assert!(!is_prime(1));
    assert!(!is_prime(12));
    assert!(is_prime(19));
    assert!(is_prime(23))
}
