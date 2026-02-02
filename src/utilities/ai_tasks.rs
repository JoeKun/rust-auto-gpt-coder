/*
 *  utilities/ai_tasks.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/25/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use super::command_line::CommandLine;
use crate::api::open_ai::request::{ call_gpt, Role, Model, Message };

use serde::de::DeserializeOwned;

// Extend AI function to encourage specific output.
fn extend_ai_function(
    ai_function: fn(&str) -> &'static str,
    function_input: &str,
) -> Message {
    let ai_function_string = ai_function(function_input);

    // Extend the string to encourage only printing the output.
    let message_content = format!("FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of functions.
    Nothing else. No commentary. Here is the input to the function: {}.
    Print out what the function will return.", 
    ai_function_string, function_input);

    // Return message.
    Message {
        role: Role::System,
        content: message_content,
    }
}

// Performs call to LLM.
pub async fn ai_task_request(
    message_context: String,
    agent_position: &String,
    agent_operation: &str,
    ai_function: for<'a> fn(&'a str) -> &'static str,
) -> String {

    // Extend AI function.
    let function_message = extend_ai_function(
        ai_function, 
        &message_context
    );

    // Print current status.
    CommandLine::print_agent_generation_message(
        agent_position, 
        agent_operation, 
    );

    // Get LLM response.
    let model = Model::GPT_5_2;
    let llm_response_result = 
        call_gpt(model.clone(), vec![function_message.clone()])
            .await;

    match llm_response_result {
        Ok(llm_response_result) => llm_response_result,
        Err(_) => {
            call_gpt(model.clone(), vec![function_message.clone()])
                .await
                .expect("Failed twice to call GPT.")
        },
    }
}

// Performs call to LLM - Decoded version.
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    message_context: String,
    agent_position: &String,
    agent_operation: &str,
    ai_function: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String = ai_task_request(
        message_context, 
        agent_position, 
        agent_operation, 
        ai_function
    )
    .await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode AI response from serde_json.");

    decoded_response
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::manager::managing_ai_functions::convert_user_input_to_goal;

    #[test]
    fn test_extending_ai_function() {
        let extended_message = extend_ai_function(
            convert_user_input_to_goal, 
            "dummy variable"
        );
        assert_eq!(extended_message.role, Role::System);
    }

    // Disable this test for global `cargo test` commands, 
    // because it requires OpenAI keys in environment variables.
    //#[tokio::test]
    #[allow(dead_code)]
    async fn test_ai_task_request() {
        let result = ai_task_request(
            "Build me a web site for making stock price API requests.".to_string(), 
            &"Managing Agent".to_string(), 
            "Defining user requirements", 
            convert_user_input_to_goal,
        )
        .await;
        
        assert!(result.len() > 20);
    }
}