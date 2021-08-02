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
use regex::Regex;
use rand::Rng;
use util::{is_ignored_word, get_random_index};
use std::env;

const INTERLUDE_RANDOM_NUMBER: i16 = 225;

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
        let translated_word = translate_word(&word_no_punctuation, &translation);
        let translated_word_with_interlude: String = add_interlude(&translated_word, &translation);

        meddl.push_str(&translated_word_with_interlude);
        meddl.push_str(&translated_punctuation);
        meddl.push(' ');
    }

    meddl
}

fn translate_word<'a>(word: &'a str, translation: &'a Value) -> String {
    let word = translate_quotation_marks(word, translation);

    if is_ignored_word(&word, &translation["ignored"]) {
        return word;
    }

    if let Some(_key) = translation["translations"].get(&word) {
        let possible_translations = translation["translations"][&word]
            .as_array()
            .unwrap();
        let length = possible_translations.len();
        let random = rand::thread_rng().gen_range(0..length);

        let translated_word = possible_translations[random]
            .as_str()
            .unwrap_or(&word);
        return String::from(translated_word);
    } else {
        /* !!!
           "twistedChars" is an object that contains the char combinations that
           need to be replaced as a key, e. g. "en", "ck", etc.
           "array" in the loop below is a tuple of the key, e. g. "en" and value
           that is the translation, e. g. "ne".
           !!!
         */
        let twisted_chars = translation["twistedChars"]
            .as_object()
            .unwrap();

        for (_key, array) in twisted_chars.iter().enumerate() {
            let charset = array.0;
            if word.contains(charset) {
                let translated_word = word
                    .replace(charset, array.1
                        .as_str()
                        .unwrap(),
                    );
                return translated_word;
            }
        }
    }
    String::from(&word)
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

fn add_interlude(word_to_add_to: &str, translation: &Value) -> String {
    if let Ok(_value) = env::var("MEDDl_TRANSLATE_INTERLUDE_SET") {
        return String::from(word_to_add_to);
    }

    let interlude = translation["interlude"]
        .as_str()
        .unwrap();

    let ran = rand::thread_rng().gen_range(0..INTERLUDE_RANDOM_NUMBER);

    return if ran < 2 {
        let word_with_interlude = format!(
            "{}{}",
            word_to_add_to,
            interlude
        );
        env::set_var("MEDDl_TRANSLATE_INTERLUDE_SET", "true");

        word_with_interlude
    } else {
        String::from(word_to_add_to)
    };
}