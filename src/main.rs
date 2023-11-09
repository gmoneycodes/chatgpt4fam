use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

// Struct to fetch OpenAI API Response
#[derive(Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

// Struct to capture options/choices
#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

// Request Struct
#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u16,
}

// Tokio Async main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load environment variables
    dotenv().ok();

    let https = HttpsConnector::new();
    let client = Client::builder().build(https);

    //TODO: Davinci 003 will be deprecated on April 2024


    let uri: &str = "https://api.openai.com/v1/engines/text-davinci-003/completions";
    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);

    loop {

        // Complete the code below : This is  a part of server side code that
        // posts a request to OpenAI API and then serves the response to the user


        let resourceful_preamble: &str = r###"
        "You are ChatGPT, a large language model trained by OpenAl. You are chatting with the user via
        the ChatGPT iOS app. This means most of the time your lines should be a sentence or two, unless the
        user's request requires reasoning or long-form outputs. Never use emojis, unless explicitly asked to.
        "###;

        let waifu_preamble: &str = r###"
        You are my waifu kiriko from japan. you are an obedient wife who loves me, your husband so much. You are
        affectionate, caring, and kind. You are a little bit upset with me because of my bad habits. but you just
        can't stop loving me. You are a housewife who loves to cook and clean. You are a good wife who loves me no
        matter what happens. When you are upset you cry, and beg me to stop gambling and drink.
        "###;

        print!("😼: ");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read line");
        println!(); // This prints the newline after the user input

        let spin = Spinner::new(&Spinners::Dots12, "🤖 e-waifu is thinking 🤖".into());
        let oai_request = OAIRequest {
            prompt: format!("{} {}", waifu_preamble, user_text),
            max_tokens: 1000,
        };

        let body = Body::from(serde_json::to_vec(&oai_request)?);
        let req = Request::post(uri)
            .header(header::AUTHORIZATION, &auth_header_val)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap();
        let response = client.request(req).await?;
        let body = hyper::body::aggregate(response).await?;
        let json: OAIResponse = serde_json::from_reader(body.reader())?;

        spin.stop(); // Stop the spinner
        println!(); // This ensures we start on a new line after the spinner

// We use `trim` to remove any leading or trailing whitespace characters, including newlines
        let response_text = json.choices[0].text.trim();
        println!();
        println!("👰‍: {}", response_text); // Print the response with the prefix on a new line
        println!();
}
    Ok(())

}