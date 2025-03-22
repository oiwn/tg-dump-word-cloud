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

#[rustfmt::skip]
pub fn get_russian_stopwords() -> Vec<String> {
    vec![
        // Common Russian function words
        "и", "в", "не", "на", "я", "быть", "он", "с", "что", "а", 
        "по", "это", "она", "этот", "к", "но", "они", "мы", "как", "из", 
        "у", "который", "то", "за", "свой", "весь", "год", "от", "так", "о", 
        "для", "ты", "же", "все", "тот", "мочь", "вы", "человек", "такой", "его", 
        "или", "один", "бы", "время", "если", "сам", "когда", "еще", "другой", "такая", 
        "ее", "во", "да", "наш", "себя", "ни", "два", "более", "нет", "уже", 
        "вот", "ну", "чтобы", "чтоб", "до", "вас", "нибудь", "ли", "её", "их",
        "там", "потом", "себе", "под", "ж", "кто", "этого", "какой", "можно", "даже",
        "чем", "со", "ним", "тут", "того", "надо", "тоже", "какая", "при", "том",
        "меня", "точно", "будут", "можешь", "свои", "всех", "понял", "наверное",
        "тебя", "какой-то", "хорошо", "недавно", "равно", "правда", "эту", "вам",
        
        // Words from your output
        "https", "можно", "нет", "даже", "надо", "тоже", "чем", "чтобы", "мне", "вообще",
        "сейчас", "без", "раз", "ещё", "очень", "больше", "вроде", "нужно", "много", "потом",
        "кто", "под", "лет", "потому", "них", "через", "работы", "тебе", "который", "этом",
        "время", "которые", "типа", "что-то", "лучше", "такое", "пока", "этого", "писать", 
        "опыт", "более", "код", "том", "себе", "ли", "сам", "такой", "сделать", "почему",
        "хотя", "тогда", "работать", "делать", "того", "конечно", "знаю", "деньги", "какой",
        "зачем", "кстати", "ничего", "работает", "года", "него", "люди", "всего", "никто",
        "хз", "скорее", "прям", "например", "кажется", "проект", "один", "тем", "человек",
        "такие", "обычно", "поэтому", "ни", "компании", "значит", "могут", "либо", "опыта",
        "меньше", "работу", "теперь", "давно", "рф", "денег", "сколько", "думаю", "работа",
        "интересно", "людей", "чего",
        
        // Technology-specific words you might want to keep
        // Comment these out if you want to include them in your word cloud
        // "rust", "раст", "расте",
        
        // Additional common words
        "просто", "будет", "ведь", "может", "где", "только", "когда", "некоторые", "был", "была",
        "были", "было", "быть", "есть", "иметь", "еще", "же", "здесь", "куда", "нас",
        "нам", "надо", "нужно", "также", "зачем", "почему", "мой", "твой", "свой", "ваш",
        "наш", "весь", "всю", "всё", "всем", "всеми", "вся", "сюда", "туда", "эта",
        "эти", "этим", "этими", "про", "как-то", "какие", "какими", "какого", "какому", "какой",
        "ими", "им", "ей", "него", "неё", "ней", "ему", "после", "перед", "между",
        "через", "над", "под", "около", "мимо", "против", "для", "вместо", "кроме", "сквозь",
        "вдоль", "поперек", "насчет", "вследствие", "благодаря", "ради", "хотя", "несмотря", "вопреки"
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
