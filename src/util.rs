use serde_json::Value;
use rand::Rng;

pub(crate) fn is_ignored_word(word: &str, ignored_words: &Value) -> bool {
    let ignored_words = ignored_words
        .as_array()
        .unwrap();

    let word = serde_json::to_value(
        word
            .to_lowercase()
    )
        .unwrap();
    if ignored_words.contains(&word) {
        return true;
    }

    false
}

pub(crate) fn get_random_index(vec: &Vec<Value>) -> usize {
    let len = vec.len();
    rand::thread_rng().gen_range(0..len)
}