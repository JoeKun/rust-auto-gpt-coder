/*
 *  api/open_ai/request.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/25/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use crate::api::open_ai::response::APIResponse;

use dotenv::dotenv;
use reqwest::Client;
use reqwest::header::{ HeaderMap, HeaderValue };
use serde::Serialize;
use std::env;

/// The role for a message to GPT.
#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Role {
    User,
    System,
}

/// A message for GPT.
#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// The model to use for the OpenAI API.
#[derive(Debug, Serialize, Clone, PartialEq)]
#[allow(dead_code, non_camel_case_types)]
pub enum Model {
    #[serde(rename = "gpt-5.2")]
    GPT_5_2,
    #[serde(rename = "gpt-5.2-pro")]
    GPT_5_2_Pro,
    #[serde(rename = "gpt-5")]
    GPT_5,
    #[serde(rename = "gpt-5-mini")]
    GPT_5_Mini,
    #[serde(rename = "gpt-5-nano")]
    GPT_5_Nano,
}

/// A chat completion for the OpenAI API.
#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion {
    pub model: Model,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

/// Calls OpenAI API with messages for a GPT model.
pub async fn call_gpt(
    model: Model,
    messages: Vec<Message>,
) -> Result<String, Box<dyn std::error::Error + Send>> {

    // Extract API Key information.
    dotenv().ok();
    let api_key = env::var("OPEN_AI_KEY")
        .expect("OPEN_AI_KEY not found in environment variables.");
    let api_org = env::var("OPEN_AI_ORG")
        .expect("OPEN_AI_ORG not found in environment variables.");

    // Confirm endpoint.
    let url = "https://api.openai.com/v1/chat/completions";

    // Create HTTP headers.
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION, 
        HeaderValue::from_str(format!("Bearer {}", api_key).as_str())
                .map_err(|error| -> Box<dyn std::error::Error + Send> {
                    Box::new(error) 
                })?
    );
    headers.insert(
        "OpenAI-Organization", 
        HeaderValue::from_str(api_org.as_str())
                .map_err(|error| -> Box<dyn std::error::Error + Send> {
                    Box::new(error) 
                })?
    );

    // Create client.
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|error| -> Box<dyn std::error::Error + Send> {
            Box::new(error) 
        })?;

    // Create chat completion.
    let chat_completion = ChatCompletion {
        model: model,
        messages: messages,
        temperature: 0.1,
    };

    // Extract API response.
    let response: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|error| -> Box<dyn std::error::Error + Send> {
            Box::new(error) 
        })?
        .error_for_status()
        .map_err(|error| -> Box<dyn std::error::Error + Send> {
            Box::new(error) 
        })?
        .json()
        .await
        .map_err(|error| -> Box<dyn std::error::Error + Send> {
            Box::new(error) 
        })?;
    
    // Return response.
    Ok(response.choices[0].message.content.clone())
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::open_ai::request::Role;

    // Disable this test for global `cargo test` commands, 
    // because it requires OpenAI keys in environment variables.
    //#[tokio::test]
    #[allow(dead_code)]
    async fn test_call_gpt() {
        let message = Message {
            role: Role::User,
            content: "Hi there, this is a test. Give me a short response.".to_string(),
        };
        let messages = vec![message];

        let response = call_gpt(Model::GPT_5_2, messages)
            .await;
        match response {
            Ok(response_string) => {
                dbg!(response_string);
                assert!(true);
            },
            Err(error) => {
                println!("Failed to call GPT with error: {:?}", error);
                assert!(false);
            }
        }
    }
}