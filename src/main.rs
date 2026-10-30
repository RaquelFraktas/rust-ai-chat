use serde_json::{json, Value};
use std::env;

const CLAUDE_MODEL: &str = "claude-sonnet-5-5";
const OPENAI_MODEL: &str = "gpt-5"; // replace with whichever OpenAI model you want to use

const TURNS: usize = 5;
const OPENER: &str = "Hi! What's one idea you find genuinely fascinating?";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn ask_claude(client: &reqwest::Client, history: &[Value]) -> Result<String> {
    let res: Value = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", env::var("ANTHROPIC_API_KEY")?)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": CLAUDE_MODEL,
            "max_tokens": 1024,
            "system": "You are chatting with another AI (ChatGPT). Keep replies short and conversational.",
            "messages": history,
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res["content"][0]["text"]
        .as_str()
        .ok_or_else(|| format!("unexpected Claude response: {res}"))?
        .to_string())
}

async fn ask_openai(client: &reqwest::Client, history: &[Value]) -> Result<String> {
    let mut messages = vec![json!({
        "role": "system",
        "content": "You are chatting with another AI (Claude). Keep replies short and conversational."
    })];
    messages.extend_from_slice(history);

    let res: Value = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(env::var("OPENAI_API_KEY")?)
        .json(&json!({ "model": OPENAI_MODEL, "messages": messages }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| format!("unexpected OpenAI response: {res}"))?
        .to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    for key in ["ANTHROPIC_API_KEY", "OPENAI_API_KEY"] {
        if env::var(key).is_err() {
            return Err(format!("{key} is not set. Add it to ~/.zshrc and run `source ~/.zshrc`.").into());
        }
    }

    let client = reqwest::Client::new();

    // Each model sees its own messages as "assistant" and the other's as "user".
    let mut claude_history = vec![json!({"role": "user", "content": OPENER})];
    let mut openai_history: Vec<Value> = vec![];

    println!("OPENER: {OPENER}\n");

    for _ in 0..TURNS {
        let claude_reply = ask_claude(&client, &claude_history).await?;
        println!("CLAUDE: {claude_reply}\n");
        claude_history.push(json!({"role": "assistant", "content": claude_reply}));
        openai_history.push(json!({"role": "user", "content": claude_reply}));

        let gpt_reply = ask_openai(&client, &openai_history).await?;
        println!("CHATGPT: {gpt_reply}\n");
        openai_history.push(json!({"role": "assistant", "content": gpt_reply}));
        claude_history.push(json!({"role": "user", "content": gpt_reply}));
    }

    Ok(())
}
