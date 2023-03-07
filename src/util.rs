use serde_json::Value;
use rand::Rng;

pub(crate) fn is_ignored_word(word: &str, translation: &Value) -> bool {
    let ignored_words = translation["ignored"]
        .as_array()
        .unwrap();

    let word = serde_json::to_value(
        word
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

pub(crate) fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    let capitalized: String = chars
        .next()
        .unwrap()
        .to_uppercase()
        .collect::<Vec<char>>()
        .iter()
        .collect();
    let index = match &capitalized.len() {
        1 => 1,
        2 => 2,
        _ => 1
    };
    return capitalized + word.split_at(index).1;
}

#[cfg(feature = "interlude")]
pub(crate) fn is_one_percent_chance() -> bool {
    let random = rand::thread_rng().gen_range(0..100);

    if random == 1 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    mod capitalize_word {
        use crate::util::capitalize_word;

        #[test]
        fn should_capitalize_umlaut_correctly() {
            assert_eq!(capitalize_word("österreich"), "Österreich");
        }

        #[test]
        fn should_capitalize_another_umlaut_correctly() {
            assert_eq!(capitalize_word("ätzend"), "Ätzend");
        }

        #[test]
        fn should_capitalize_correctly() {
            assert_eq!(capitalize_word("doppelhaushälfte"), "Doppelhaushälfte");
        }
    }
}