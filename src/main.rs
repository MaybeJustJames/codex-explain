use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;
use std::io::BufRead;
use std::{env, io};

#[derive(Deserialize)]
struct Completion {
    text: String,
    index: u32,
    logprobs: Option<f32>,
    finish_reason: String,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Deserialize)]
struct CompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Completion>,
    usage: Usage,
}

fn main() {
    match env::var("OPENAI_API_KEY") {
        Ok(key) => run_app(&key, read_input().as_str()),
        Err(_) => eprintln!("Need an API key in environment variable: OPENAI_API_KEY"),
    }
}

fn run_app(key: &str, prompt: &str) {
    let body = json!({
        "model": "text-davinci-002",
        "prompt": prompt,
        "temperature": 0,
        "max_tokens": 50
    });

    let client = Client::new();
    let res = client
        .post("https://api.openai.com/v1/completions")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .bearer_auth(key)
        .json(&body)
        .send();
    match res {
        Ok(response) => {
            if response.status().is_success() {
                let completion: CompletionResponse = response.json().unwrap();
                println!("{}", completion.choices[0].text);
            } else {
                eprintln!("Request failed");
            }
        }
        Err(_) => eprintln!("Error occurred"),
    }
}

fn read_input() -> String {
    let lines = io::stdin().lock().lines();
    let mut input = String::new();

    for line in lines {
        let last = line.unwrap();
        if last.is_empty() {
            break;
        }
        input.push_str(&last)
    }

    input
}
