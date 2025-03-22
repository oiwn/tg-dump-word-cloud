use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug)]
pub struct SimpleMessage {
    pub username: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextEntity {
    pub r#type: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub r#type: String,
    pub count: i32,
    pub emoji: String,
    #[serde(default)]
    pub recent: Vec<ReactionUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionUser {
    pub from: Option<String>,
    pub from_id: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub r#type: String,
    pub date: String,
    pub date_unixtime: String,
    #[serde(default)]
    pub edited: Option<String>,
    #[serde(default)]
    pub edited_unixtime: Option<String>,
    pub from: Option<String>,
    pub from_id: Option<String>,
    #[serde(default)]
    pub reply_to_message_id: Option<i64>,
    // Handle text which can be a plain string or an array of text entities
    #[serde(default)]
    pub text: serde_json::Value,
    #[serde(default)]
    pub text_entities: Vec<TextEntity>,
    #[serde(default)]
    pub reactions: Vec<Reaction>,
}

pub fn read_messages<P: AsRef<Path>>(file_path: P) -> Result<Vec<Message>> {
    let content = std::fs::read_to_string(file_path)
        .with_context(|| "Failed to read file content")?;

    let mut messages = Vec::new();
    let mut start_idx = 0;

    // Iterate through the content by characters, not bytes
    let chars: Vec<char> = content.chars().collect();

    while start_idx < chars.len() {
        // Find the next opening brace
        let mut obj_start = None;
        for i in start_idx..chars.len() {
            if chars[i] == '{' {
                obj_start = Some(i);
                break;
            }
        }

        if let Some(start) = obj_start {
            // Find the matching closing brace
            let mut brace_count = 1;
            let mut obj_end = None;

            for i in (start + 1)..chars.len() {
                match chars[i] {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            obj_end = Some(i);
                            break;
                        }
                    }
                    _ => {}
                }
            }

            if let Some(end) = obj_end {
                // We found a complete JSON object
                let json_str: String = chars[start..=end].iter().collect();

                match serde_json::from_str::<Message>(&json_str) {
                    Ok(message) => messages.push(message),
                    Err(e) => {
                        eprintln!("Warning: Failed to parse message: {}", e);
                        // Continue with next message
                    }
                }
                start_idx = end + 1;
            } else {
                // Unmatched braces, move past this opening brace
                start_idx = start + 1;
            }
        } else {
            // No more opening braces
            break;
        }
    }

    if messages.is_empty() {
        anyhow::bail!("No valid messages found in the file");
    }

    Ok(messages)
}

pub fn simplify_messages(messages: &[Message]) -> Vec<SimpleMessage> {
    messages
        .iter()
        .filter_map(|msg| {
            // Skip messages without text
            let text = extract_message_text(msg);
            if text.is_empty() {
                return None;
            }

            // Get username or a placeholder if it's missing
            let username = match &msg.from {
                Some(name) => name.clone(),
                None => match &msg.from_id {
                    Some(id) => id.clone(),
                    None => "anonymous".to_string(),
                },
            };

            Some(SimpleMessage { username, text })
        })
        .collect()
}

pub fn extract_message_text(message: &Message) -> String {
    match &message.text {
        serde_json::Value::String(text) => text.clone(),
        serde_json::Value::Array(parts) => {
            let mut result = String::new();
            for part in parts {
                if let serde_json::Value::Object(obj) = part {
                    if let Some(serde_json::Value::String(text)) = obj.get("text")
                    {
                        result.push_str(text);
                    }
                } else if let serde_json::Value::String(text) = part {
                    result.push_str(text);
                }
            }
            result
        }
        _ => {
            // If there's no text field or it's in an unexpected format,
            // try to use text_entities
            if !message.text_entities.is_empty() {
                message
                    .text_entities
                    .iter()
                    .map(|entity| entity.text.clone())
                    .collect::<Vec<_>>()
                    .join("")
            } else {
                String::new()
            }
        }
    }
}
