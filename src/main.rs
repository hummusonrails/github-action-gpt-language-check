use reqwest::Client;
use serde::{Serialize};
use serde_json::Value;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct SuggestionError(String);

impl fmt::Display for SuggestionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SuggestionError {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let review = review_text().await?;
    println!("{}", review);

    Ok(())
}

#[derive(Serialize)]
struct Prompt {
    prompt: String,
    max_tokens: u32,
}

// Review text for potentially discriminatory language
async fn review_text() -> Result<String, Box<dyn Error>> {
    // Get the path to the markdown file from the command line
    let args: Vec<String> = std::env::args().collect();
    // Check that the user provided a path to a markdown file
    if args.len() != 2 {
        println!("Usage: {} <path_to_markdown_file>", args[0]);
        std::process::exit(1);
    }
    // Read the markdown file
    let file_path = &args[1];
    let text = std::fs::read_to_string(file_path)?;
    // Get the OpenAI API key from the environment
    let api_key = std::env::var("OPENAI_API_KEY")?;
    // Call the OpenAI API
    let review = call_openai_api(&api_key, &text, "https://api.openai.com").await?;
    println!("{}", review);

    Ok(review)
}

async fn call_openai_api(api_key: &str, text: &str, base_url: &str) -> Result<String, Box<dyn Error>> {
    // Create a new HTTP client
    let client = Client::new();
    // Define the prompt text
    let prompt = format!(
        "Review the following text for potentially discriminatory language, including but not limited to ableism, ageism, racism, antisemitism, islamophobia, and likewise: {}",
        text
    );

    // Define the request body with the model parameter and prompt text
    let request_body = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": 100
    });

    // Send the request to the OpenAI API
    let response = client
    .post(&format!("{}/v1/chat/completions", base_url))
    .header("Authorization", format!("Bearer {}", api_key))
    .json(&request_body)
    .send()
    .await?
    .json::<Value>()
    .await?;

    // Extract the suggestion from the response
    let choices = response["choices"].as_array().ok_or(SuggestionError("Missing choices field".to_string()))?;
    let suggestion = choices
        .get(0)
        .and_then(|choice| choice["message"]["content"].as_str()) // Updated path to extract content
        .ok_or(SuggestionError("Missing suggestion text".to_string()))?
        .trim()
        .to_owned();

    Ok(suggestion)
}

#[cfg(test)]
mod tests {
    use super::call_openai_api;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_review_without_discriminatory_language() {
        let _m = mock("POST", "/v1/chat/completions")
            .with_status(200)
            .with_body(r#"{
                "choices": [
                    {
                        "finish_reason": "stop",
                        "index": 0,
                        "message": {
                            "content": "There does not appear to be any discriminatory language in this text.",
                            "role": "assistant"
                        }
                    }
                ]
            }"#)
            .create();

        let api_key = "fake_api_key";
        let text = "This is a normal sentence without any discriminatory language.";
        let review = call_openai_api(api_key, text, &server_url()).await.expect("API call failed");
        assert_eq!(review, "There does not appear to be any discriminatory language in this text.");
    }

    #[tokio::test]
    async fn test_review_with_discriminatory_language() {
        let _m = mock("POST", "/v1/chat/completions")
            .with_status(200)
            .with_body(r#"{
                "choices": [
                    {
                        "finish_reason": "stop",
                        "index": 0,
                        "message": {
                            "content": "The text contains discriminatory language. Suggested alternative: newcomers",
                            "role": "assistant"
                        }
                    }
                ]
            }"#)
            .create();

        let api_key = "fake_api_key";
        let text = "This is an example sentence with discriminatory language: stupid beginners";
        let review = call_openai_api(api_key, text, &server_url()).await.expect("API call failed");
        let expected_result = "The text contains discriminatory language. Suggested alternative: newcomers";
        assert_eq!(review, expected_result);
    }
}
                    
