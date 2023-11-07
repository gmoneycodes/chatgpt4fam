use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize,Serialize};
use spinners::{Spinner,Spinners};
use std::env;
use std::io::{stdin, stdout, Write};


// Struct to fetch OpenAI API Response
#[derive(Deserialize, Debug)]
struct OAIResponse{
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>
}

// Struct to capture options/choices
#[derive(Deserialize, Debug)]
struct OAIChoices{
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

// Request Struct
#[derive(Serialize, Debug)]
struct OAIRequest{
    prompt: String,
    max_tokens: u16,
}

// Tokio Async main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + send + Sync>> {

    // Load environment variables
    dotenv().ok();

    // Client
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);

    /*
    turbo = gpt-3.5-turbo
    TODO: Davinci 003 will be deprecated on April 2024
    */

    let resourceful_preamble: &str = r###"
    "You are ChatGPT, a large language model trained by OpenAl. You are chatting with the user via
    the ChatGPT iOS app. This means most of the time your lines should be a sentence or two, unless the
    user's request requires reasoning or long-form outputs. Never use emojis, unless explicitly asked to.
    "###;

    let uri: &str = "https://api.openai.com/v1/engines/text-davinci-003/completions";


}