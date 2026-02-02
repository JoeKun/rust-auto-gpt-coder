/*
 *  model/project.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/31/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use super::endpoint_route::EndpointRoute;
use super::project_scope::ProjectScope;

use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
    pub description: String,
    pub scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<EndpointRoute>>,
}
