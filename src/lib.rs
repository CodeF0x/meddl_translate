//! Translate German to (almost) perfect Meddlfrängisch.
//!
//! # Usage
//!
//! ```
//! fn main() {
//!     println!("{}", meddl_translate::translate("Hallo"));
//! }
//! ```
//!
//! # Examples
//!
//! ```shell
//! $ cargo run --example hallo
//! ```
//! ```shell
//! $ cargo run --example langer-text
//! ```
//!
//! # Benchmark
//!
//! ```shell
//! $ cargo bench
//! ```
//!
//! You need to use Rust nightly for running the benchmark.

use serde_json::Value;
use regex::Regex;
use rand::Rng;

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

        meddl.push_str(&translated_word);
        meddl.push_str(translated_punctuation);
        meddl.push(' ');
    }

    meddl
}

fn translate_word<'a>(word: &'a str, translation: &'a Value) -> String {
    let word = translate_quotation_marks(word, translation);

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
                        .unwrap()
                    );
                return translated_word;
            }
        }
    }
    String::from(&word)
}

fn translate_punctuation<'a>(punctuation: &'a str, translation: &'a Value) -> &'a str {
    return match punctuation {
        "." => {
            let dot_pool = translation["dot"]
                .as_array()
                .unwrap();
            let random = get_random_index(dot_pool);
            let translated_dot = dot_pool[random]
                .as_str()
                .unwrap();

            translated_dot
        },
        "!" => {
            let exclamation_mark_pool = translation["exclamationMark"]
                .as_array()
                .unwrap();
            let random = get_random_index(exclamation_mark_pool);
            let translated_exclamation_mark = exclamation_mark_pool[random]
                .as_str()
                .unwrap();

            translated_exclamation_mark
        },
        "?" => {
            let question_mark_pool = translation["questionMark"]
                .as_array()
                .unwrap();
            let random = get_random_index(question_mark_pool);
            let translated_question_mark = question_mark_pool[random]
                .as_str()
                .unwrap();

            translated_question_mark
        }
        _ => punctuation
    }
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

fn get_random_index(vec: &Vec<Value>) -> usize {
    let len = vec.len();
    rand::thread_rng().gen_range(0..len)
}