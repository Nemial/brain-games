use crate::games::start_game;
use rand::Rng;
use std::collections::BTreeMap;

const GAME_NAME: &str = "Brain Progression";
const GAME_DESCRIPTION: &str = "What number is missing in the progression?";

/// # Panics
/// This function will panic if it does not get value progression.
pub fn start() {
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

    for value in progression.values() {
        question.push_str(value);
        question.push(' ');
    }

    start_game(GAME_NAME, GAME_DESCRIPTION, &answer, &question);
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

#[cfg(test)]
mod tests {
    use crate::games::progression::gen_progression;
    use std::collections::BTreeMap;

    #[test]
    fn check_gen_progression() {
        let mut expected = BTreeMap::new();
        expected.insert(0, "1".to_string());
        expected.insert(1, "2".to_string());
        expected.insert(2, "3".to_string());

        assert_eq!(gen_progression(1, 3, 1), expected);
        assert_eq!(gen_progression(1, 0, 10), BTreeMap::new())
    }
}
