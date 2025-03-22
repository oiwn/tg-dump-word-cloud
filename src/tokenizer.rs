use crate::parse::SimpleMessage;
use regex::Regex;

#[derive(Debug)]
pub struct Token {
    pub word: String,
}

pub fn tokenize_messages(
    messages: &[SimpleMessage],
    min_length: usize,
) -> Vec<Token> {
    // Regex to match valid words (letters and some special characters)
    // This will exclude emojis, punctuation, and other symbols
    let word_regex = Regex::new(r"[\p{L}\p{N}_-]+").unwrap();

    let mut tokens = Vec::new();

    for message in messages {
        // Find all word matches in the message text
        for capture in word_regex.find_iter(&message.text) {
            let word = capture.as_str().to_lowercase();

            // Skip words that are too short
            if word.len() < min_length {
                continue;
            }

            tokens.push(Token { word });
        }
    }

    tokens
}

// Optional: Function to filter tokens by language-specific stop words
pub fn filter_stop_words(
    tokens: Vec<Token>,
    stop_words: &[String],
) -> Vec<Token> {
    tokens
        .into_iter()
        .filter(|token| !stop_words.contains(&token.word))
        .collect()
}

// Optional: Function to stem words for better counting
pub fn stem_tokens(tokens: Vec<Token>, lang: &str) -> Vec<Token> {
    use rust_stemmers::{Algorithm, Stemmer};

    // Select stemmer based on language
    let stemmer = match lang.to_lowercase().as_str() {
        "ru" => Stemmer::create(Algorithm::Russian),
        "en" => Stemmer::create(Algorithm::English),
        // Add other languages as needed
        _ => Stemmer::create(Algorithm::English), // Default to English
    };

    tokens
        .into_iter()
        .map(|token| Token {
            word: stemmer.stem(&token.word).to_string(),
        })
        .collect()
}

pub fn count_words(tokens: &[Token]) -> std::collections::HashMap<String, usize> {
    let mut word_counts = std::collections::HashMap::new();

    for token in tokens {
        *word_counts.entry(token.word.clone()).or_insert(0) += 1;
    }

    word_counts
}

pub fn get_russian_stopwords() -> Vec<String> {
    vec![
        "не",
        "на",
        "что",
        "это",
        "как",
        "но",
        "то",
        "по",
        "для",
        "если",
        "от",
        "так",
        "за",
        "из",
        "же",
        "или",
        "бы",
        "вы",
        "да",
        "я",
        "в",
        "и",
        "с",
        "а",
        "о",
        "к",
        "у",
        "во",
        "со",
        "об",
        "ну",
        "он",
        "она",
        "оно",
        "они",
        "мы",
        "ты",
        "вы",
        "его",
        "ее",
        "их",
        "мой",
        "твой",
        "наш",
        "ваш",
        "меня",
        "тебя",
        "нас",
        "вас",
        "был",
        "была",
        "было",
        "были",
        "есть",
        "быть",
        "буду",
        "будет",
        "будут",
        "при",
        "про",
        "до",
        "после",
        "всех",
        "всё",
        "все",
        "весь",
        "этот",
        "эта",
        "это",
        "эти",
        "там",
        "тут",
        "здесь",
        "где",
        "когда",
        "только",
        "уже",
        "еще",
        "вот",
        "может",
        "просто",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
