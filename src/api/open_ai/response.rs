/*
 *  api/open_ai/request.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/25/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct APIMessage {
    pub content: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct APIChoice {
    pub message: APIMessage,
}

#[derive(Debug, Deserialize, Clone)]
pub struct APIResponse {
    pub choices: Vec<APIChoice>,
}