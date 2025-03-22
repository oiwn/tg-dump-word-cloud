use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use wordcloud_rs::*;

mod parse;
mod tokenizer;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Generate a word cloud from Telegram chat export",
    after_help = "Example: tg-dump-word-cloud -i telegram_dump.json -o wordcloud.png --lang ru"
)]
struct Args {
    /// Input file containing Telegram chat dump in JSON format
    #[arg(short, long, required = true)]
    input: PathBuf,

    /// Output file for the word cloud image (PNG)
    #[arg(short, long, default_value = "wordcloud.png")]
    output: PathBuf,

    /// Minimum word length to include
    #[arg(short, long, default_value_t = 3)]
    min_length: usize,

    /// Maximum number of words to include in the cloud
    #[arg(long, default_value_t = 100)]
    max_words: usize,

    /// Language code for stemming (en, ru, etc.)
    #[arg(long, default_value = "en")]
    lang: String,

    /// List of users to include (default: all)
    #[arg(short, long)]
    users: Option<Vec<String>>,

    /// Skip messages before this date (format: YYYY-MM-DD)
    #[arg(long)]
    from_date: Option<String>,

    /// Skip messages after this date (format: YYYY-MM-DD)
    #[arg(long)]
    to_date: Option<String>,

    /// List of stop words to exclude
    #[arg(long)]
    stop_words: Option<Vec<String>>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Reading messages from {:?}", args.input);
    let messages = parse::read_messages(&args.input)?;
    println!("Found {} messages", messages.len());

    let simple_messages = parse::simplify_messages(&messages);
    println!("Extracted {} messages with text", simple_messages.len());
    let len = simple_messages.len();
    // println!("Samples: {:?}", &simple_messages[len.saturating_sub(5)..]);

    println!("Extracting text tokens");
    let tokens =
        tokenizer::tokenize_messages(&simple_messages, args.min_length.max(4));
    println!("Extracted {} tokens", tokens.len());

    // Filter Russian stopwords
    let stop_words = tokenizer::get_russian_stopwords();
    // let stop_words = args.stop_words.unwrap_or_default();
    let filtered_tokens = tokenizer::filter_stop_words(tokens, &stop_words);
    println!(
        "After filtering stop words: {} tokens",
        filtered_tokens.len()
    );

    let stemmed_tokens = tokenizer::stem_tokens(filtered_tokens, &args.lang);
    println!("After stemming: {} tokens", stemmed_tokens.len());

    let word_counts = tokenizer::count_words(&stemmed_tokens);
    println!("Found {} unique words", word_counts.len());
    println!("{:?}", word_counts);

    // Convert to wordcloud-rs Token format
    let mut wc_tokens = Vec::new();

    // Sort words by frequency and take top N words
    let mut words: Vec<_> = word_counts.into_iter().collect();
    words.sort_by(|a, b| b.1.cmp(&a.1));
    words.truncate(args.max_words);

    let python_data_path = args.output.with_extension("txt");
    println!(
        "Saving word data for Python to {}",
        python_data_path.display()
    );
    save_word_counts_for_python(&words, &python_data_path)?;

    // Print top words being used for the cloud
    println!("Top 40 words:");
    for (i, (word, count)) in words.iter().take(40).enumerate() {
        println!("{}. {} ({})", i + 1, word, count);
    }

    // Convert to wordcloud tokens
    for (word, count) in words {
        wc_tokens.push((Token::Text(word), count as f32));
    }

    println!("Generating word cloud with {} words", wc_tokens.len());
    let wc = WordCloud::new().font("DejaVu Sans").generate(wc_tokens);

    println!("Saving word cloud to {}", args.output.display());
    wc.save(&args.output)?;

    println!("Word cloud generated at: {}", args.output.display());
    Ok(())
}

fn save_word_counts_for_python(
    words: &[(String, usize)],
    output_path: &std::path::Path,
) -> Result<()> {
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    for (word, count) in words {
        writeln!(writer, "{} {}", word, count)?;
    }

    Ok(())
}
