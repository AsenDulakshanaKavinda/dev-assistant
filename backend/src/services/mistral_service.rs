use reqwest::Client;


use crate::models::mistral::{Message, MistralRequest, MistralResponse};

pub async fn send_to_mistral(user_input: String) -> Result<String, String> {
    
    let api_key = std::env::var("MISTRAL_API_KEY").unwrap();

    // create a new client
    let client = Client::new();

    // create a request body
    let request_body = MistralRequest {
        model: "mistral-small-latest".to_string(),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: user_input,
            }
        ],
    };

    // send request to the mistral
    let response = client
        .post("https://api.mistral.ai/v1/chat/completions")
        .bearer_auth(&api_key)
        .json(&request_body)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    // convert response to json
    let response_body: MistralResponse = response
        .json()
        .await
        .map_err(|err| err.to_string())?;

    // return the reply
    Ok(response_body.choices[0].message.content.clone())

}



