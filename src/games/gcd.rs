use crate::games::start_game;
use rand::Rng;

const GAME_NAME: &str = "Brain GCD";
const GAME_DESCRIPTION: &str = "Find the greatest common divisor of given numbers.";
pub fn start() {
    let mut rng = rand::thread_rng();

    let first_num: usize = rng.gen_range(1..256);
    let second_num: usize = rng.gen_range(1..first_num);

    let question = format!("{} {}", first_num, second_num);
    let answer = find_gcd(first_num, second_num);

    start_game(GAME_NAME, GAME_DESCRIPTION, answer.to_string(), question);
}

fn find_gcd(first_num: usize, second_num: usize) -> usize {
    if first_num % second_num != 0 {
        find_gcd(second_num, first_num % second_num)
    } else {
        second_num
    }
}
