use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{self, stdin, stdout, Write, BufRead, BufReader};
use std::fs::File;
use serde::forward_to_deserialize_any;


// Helper function to select and read preambles
fn read_preambles_from_file() -> io::Result<Vec<(String, String)>> {
    let file = File::open("preambles.txt")?;
    let reader = BufReader::new(file);
    let mut preambles = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            preambles.push((parts[0].trim().to_string(), parts[1].trim().to_string()));
        }
    }
    Ok(preambles)
}


fn select_preamble(preambles: &[(String, String)]) -> io::Result<String> {
    for (index, (name, _)) in preambles.iter().enumerate() {
        println!("{}: {}", index + 1, name);
    }
    print!("Select a preamble: ");
    stdout().flush()?;
    let mut choice = String::new();
    stdin().read_line(&mut choice)?;
    let index: usize = choice.trim().parse().unwrap_or(1) - 1;

    Ok(preambles.get(index).map_or(String::new(), |(_, preamble)| preamble.clone()))
}


// Struct to fetch OpenAI API Response
#[derive(Deserialize, Debug)]
struct ChatResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<Choice>,
    usage: Option<Usage>
}


#[derive(Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct Usage {
    text_characters: Option<u16>,
    completion_tokens: Option<u16>,
    total_tokens: Option<u16>,
}

// Struct to capture options/choices
#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

// Request Struct
#[derive(Serialize, Debug)]
struct ChatRequest {
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

    // New OpenAI API endpoint
    let uri: &str =  "https://api.openai.com/v1/chat/completion";

    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);

    let preambles = read_preambles_from_file()?;
    let selected_preamble = select_preamble(&preambles)?;

    loop {

        // Complete the code below : This is  a part of server side code that
        // posts a request to OpenAI API and then serves the response to the user

        print!("😼: ");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read line");
        println!(); // This prints the newline after the user input

        let spin = Spinner::new(&Spinners::Dots12, "🤖 Mario is thinking 🤖".into());
        let oai_request = ChatRequest {
            prompt: format!("{} {}", selected_preamble, user_text),
            max_tokens: 1000,
        };

        let body = Body::from(serde_json::to_vec(&oai_request)?);
        let req = Request::post(uri_gpt4)
            .header(header::AUTHORIZATION, &auth_header_val)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap();

        let response = client.request(req).await?;
        let body = hyper::body::aggregate(response).await?;
        let json: ChatResponse = serde_json::from_reader(body.reader())?;

        spin.stop();
        println!(); // This ensures we start on a new line after the spinner

// We use `trim` to remove any leading or trailing whitespace characters, including newlines
        let response_text = json.choices[0].text.trim();
        println!();
        println!("👰‍: {}", response_text); // Print the response with the prefix on a new line
        println!();
}
    Ok(())

}