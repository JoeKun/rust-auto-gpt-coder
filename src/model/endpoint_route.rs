/*
 *  model/endpoint_route.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/31/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use serde::{ Deserialize, Serialize };
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HTTPMethod {
    Get,
    Patch,
    Post,
    Put,
    Delete,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EndpointRoute {
    pub is_route_dynamic: bool,
    pub method: HTTPMethod,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route: String,
}