use serde::{Deserialize, Serialize};
const api_key: &str = "sk-JM8fEs08gKa6Hs5FK7oVJuOFjyL7IpweAWe8xQbM";
const base_url: &str = "https://api.openai.com/v1/completions";

pub fn create_text_completion(prompt: &str) -> String {
    let body = RequestBody {
        model: "text-davinci-002".to_string(),
        prompt: prompt.to_string(),
        max_tokens: 64,
        temperature: 0.9,
        top_p: 1.0,
        n: 1,
        presence_penalty: 0.0,
        frequency_penalty: 0.0,
    };

    let authorization_header_value = format!("Bearer {api_key}");

    let body_text = serde_json::to_string(&body).expect("Error serializing request body.");
    println!("{}", &body_text);

    let response_text = reqwest::blocking::Client::new()
        .post(base_url)
        .header("Authorization", authorization_header_value)
        .header("Content-Type", "application/json")
        .body(body_text)
        .send()
        .expect("Problem hitting the Open AI API.")
        .text()
        .expect("Problem reading the response from the Open AI API as text.");

    println!("{}", response_text);

    let mut response: ApiResponse = serde_json::from_str(&response_text)
        .expect("Error parsing the Open AI API response into the desired Rust struct.");

    response.choices.swap_remove(0).text.trim().to_owned()
}

#[derive(Serialize, Debug)]
struct RequestBody {
    model: String,
    prompt: String,
    max_tokens: usize,
    temperature: f32,
    top_p: f32,
    n: usize,
    presence_penalty: f32,
    frequency_penalty: f32,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
}
