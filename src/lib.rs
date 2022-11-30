//! Translate German to (almost) perfect Meddlfrängisch.
//!
//! # Usage
//!
//! ```rust
//! fn main() {
//!     println!("{}", meddl_translate::translate("Hallo"));
//! }
//! ```
//!
//! # Examples
//!
//! ```shell
//! $ cargo run --example hello
//! ```
//! ```shell
//! $ cargo run --example long-text
//! ```
//!
//! # Excluding words from being translated
//!
//! ```json
//! "ignored": [
//!     "den"
//! ]
//! ```
//!
//! Example containing an ignored word:
//!
//! ```shell
//! $ cargo run --example ignored
//! ```
//!
//! # Benchmark
//!
//! ```shell
//! $ cargo bench
//! ```
//!
//! You need to use Rust nightly for running the benchmark.

mod util;

use serde_json::Value;
use regex::{Regex};
#[cfg(feature = "interlude")]
use util::{is_ignored_word, get_random_index, is_one_percent_chance};
#[cfg(not(feature = "interlude"))]
use util::{is_ignored_word, get_random_index};

fn parse_translation() -> Option<Value> {
    let translation_string = include_str!("de-oger.json");
    Some(serde_json::from_str(&translation_string).expect("Could not parse translation."))
}

/// This function translates a string slice from German to Meddlfrängisch.
///
/// # Example
///
/// ```
/// fn main() {
///     let meddl_fraengisch = meddl_translate::translate("Hallo Welt");
/// }
/// ```
pub fn translate(original: &str) -> String {
    let words: Vec<&str> = original.split(" ").collect();
    let translation: Value = parse_translation().unwrap();
    let punctuation_regex = Regex::new(r"[.,\\/#!?$%\^&\*;:{}=\-_`~()]").expect("Could not compile punctuation regex.");
    let mut meddl = String::new();

    for i in 0..words.len() {
        let punctuation = punctuation_regex
            .find(words[i])
            .map(|punc| punc.as_str())
            .unwrap_or("");
        let cow = punctuation_regex.replace_all(words[i], "");
        let mut word_no_punctuation = String::new();
        word_no_punctuation.push_str(&cow);

        let translated_punctuation = translate_punctuation(&punctuation, &translation);

        #[cfg(feature = "interlude")]
        let mut translated_word = translate_word(&word_no_punctuation, &translation);
        #[cfg(not(feature = "interlude"))]
        let translated_word = translate_word(&word_no_punctuation, &translation);

        #[cfg(feature = "interlude")]
        if is_one_percent_chance() {
            translated_word = add_interlude(&translated_word, &translation);
        }

        meddl.push_str(&translated_word);
        meddl.push_str(&translated_punctuation);
        meddl.push_str(" ");
    }

    String::from(meddl.trim())
}

fn translate_word<'a>(word: &'a str, translation: &'a Value) -> String {
    let mut word = translate_quotation_marks(word, translation);

    if is_ignored_word(&word, &translation) {
        return word;
    }

    if let Some(_key) = translation["translations"].get(&word) {
        let possible_translations = translation["translations"][&word]
            .as_array()
            .unwrap();
        let random = get_random_index(&possible_translations);

        let translated_word = possible_translations[random]
            .as_str()
            .unwrap_or(&word);
        word = String::from(translated_word);
    } else {
        word = twist_en(&word, &translation);
        
    }
    word = twist_chars(&word, &translation);

    word
}

fn twist_chars<'a>(word: &'a str, translation: &'a Value) -> String {
    let twisted_chars = translation["twistedChars"]
        .as_object()
        .unwrap();
    let is_noun = word
        .chars()
        .collect::<Vec<char>>()[0]
        .is_uppercase();
    let mut translated_word = String::from(word).to_lowercase();

    for (_key, pair) in twisted_chars.iter().enumerate() {
        let key = pair.0.to_lowercase();
        if translated_word.contains(&key) {
            translated_word = translated_word
                .replace(&key, pair.1
                    .as_str()
                    .unwrap()
                    .to_lowercase()
                    .as_str(),
                );
        }
    }

    if is_noun {
        return translated_word[0..1].to_uppercase() + &translated_word[1..];
    }

    translated_word
}

fn twist_en<'a>(word: &'a str, translation: &'a Value) -> String {
    let mut twisted = String::from(word);

    let ens = translation["en"]
        .as_object()
        .unwrap();

    for (_key, array) in ens.iter().enumerate() {
        let to_replace = array.0;
        if word.ends_with(to_replace) {
            let position = word.rfind(to_replace).unwrap();

            twisted
                .replace_range(position..word.len(), array.1
                    .as_str()
                    .unwrap()
                )
        }
    }

    twisted
}

fn translate_punctuation<'a>(punctuation: &'a str, translation: &'a Value) -> String {
    return match punctuation {
        "." => {
            let dot_pool = translation["dot"]
                .as_array()
                .unwrap();
            let random = get_random_index(dot_pool);
            let translated_dot = dot_pool[random]
                .as_str()
                .unwrap();

            String::from(translated_dot)
        }
        "!" => {
            let exclamation_mark_pool = translation["exclamationMark"]
                .as_array()
                .unwrap();
            let random = get_random_index(exclamation_mark_pool);
            let translated_exclamation_mark = exclamation_mark_pool[random]
                .as_str()
                .unwrap();

            String::from(translated_exclamation_mark)
        }
        "?" => {
            let question_mark_pool = translation["questionMark"]
                .as_array()
                .unwrap();
            let random = get_random_index(question_mark_pool);
            let translated_question_mark = question_mark_pool[random]
                .as_str()
                .unwrap();

            String::from(translated_question_mark)
        }
        _ => String::from(punctuation)
    };
}

fn translate_quotation_marks(word: &str, translation: &Value) -> String {
    if word.starts_with("\"") {
        return word.replacen("\"", translation["quotationMark"]
            .as_str()
            .unwrap(),
        1);
    }
    String::from(word)
}

#[cfg(feature = "interlude")]
fn add_interlude(word_to_add_to: &str, translation: &Value) -> String {
    let interlude = translation["interlude"]
        .as_str()
        .unwrap();

        let word_with_interlude = format!(
            "{}{}",
            word_to_add_to,
            interlude
        );

        word_with_interlude
}

#[cfg(test)]
mod tests {
    mod translate {
        use crate::translate;

        #[test]
        fn should_translate_everything() {
            assert_eq!(translate("Der Meddltranslator wurde in Rust programmiert"), "Der Meddldranslador wurde in Rusd brogrammierd");
        }
    }

    mod translate_word {
        use super::super::*;
        #[test]
        fn should_ignore_word() {
            let translation = serde_json::from_str("{\"ignored\": [\"whatever\"], \"translations\": { \"whatever\": [\"something\"]}}").unwrap();

            assert_eq!(translate_word("whatever", &translation), "whatever");
        }

        #[test]
        fn should_ignore_upper_case_word() {
            let translation = serde_json::from_str("{\"ignored\": [\"den\"], \"translations\": { \"Den\": [\"something\"]}}").unwrap();

            assert_eq!(translate_word("Den", &translation), "Den");
        }

        #[test]
        fn should_translate_word() {
            let translation = serde_json::from_str("{\"translations\": { \"whatever\": [\"something\"]}, \"ignored\": [], \"en\": {}, \"twistedChars\": {}}").unwrap();

            assert_eq!(translate_word("whatever", &translation), "something");
        }

        #[test]
        fn should_translate_nn_correctly() {
            let translation = serde_json::from_str("{\"translations\": { \"wenn\": [\"wen\"]}, \"ignored\": [], \"en\": {}, \"twistedChars\": {}}").unwrap();

            assert_eq!(translate_word("wenn", &translation), "wen");
        }
    }

    mod twist_chars {
        use serde_json::Value;
        use crate::twist_chars;

        #[test]
        fn should_twist_chars() {
            let translation = serde_json::from_str("{\"twistedChars\": {\"ck\": \"gg\"}}").unwrap();

            assert_eq!(twist_chars("wicked", &translation), "wigged");
        }

        #[test]
        fn should_twist_multiple_chars() {
            let translation: Value = serde_json::from_str("{\"twistedChars\": {\"z\": \"ds\", \"p\": \"b\"}}").unwrap();

            assert_eq!(twist_chars("Pommespanzer", &translation), "Bommesbandser");
        }
    }

    mod twist_en {
        use crate::twist_en;

        #[test]
        fn should_twist_en_end_of_word() {
            let translation = serde_json::from_str("{\"en\": {\"en!\": \"ne!\"}, \"ignored\": []}").unwrap();

            assert_eq!(twist_en("laufen!", &translation), "laufne!");
        }

        #[test]
        fn should_twist_en_ignore_char_within() {
            let translation = serde_json::from_str("{\"en\": {\"en\": \"ne\"}, \"ignored\": []}").unwrap();

            assert_eq!(twist_en("denken", &translation), "denkne");
        }
    }

    mod translate_punctuation {
        use crate::translate_punctuation;

        #[test]
        fn should_translate_punctuation_dot() {
            let translation = serde_json::from_str("{\"dot\": [\" dot suffix.\"]}").unwrap();

            assert_eq!(translate_punctuation(".", &translation), " dot suffix.");
        }

        #[test]
        fn should_translation_punctuation_exclamation_mark() {
            let translation = serde_json::from_str("{\"exclamationMark\": [\" exclamation mark suffix!\"]}").unwrap();

            assert_eq!(translate_punctuation("!", &translation), " exclamation mark suffix!");
        }

        #[test]
        fn should_translate_punctuation_question_mark() {
            let translation = serde_json::from_str("{\"questionMark\": [\" question mark suffix?\"]}").unwrap();

            assert_eq!(translate_punctuation("?", &translation), " question mark suffix?");
        }

        #[test]
        fn should_translate_punctuation_return_anything_else() {
            let translation = serde_json::from_str("{}").unwrap();

            assert_eq!(translate_punctuation("~", &translation), "~");
        }
    }

    mod translate_quotation_marks {
        use crate::translate_quotation_marks;

        #[test]
        fn should_translate_quotation_marks() {
            let translation = serde_json::from_str("{\"quotationMark\":\"I cite: \\\"\"}").unwrap();

            assert_eq!(translate_quotation_marks("\"word\"", &translation), "I cite: \"word\"");
        }

    }
    
}