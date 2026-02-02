/*
 *  agents/common/traits.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/31/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use crate::model::project::Project;

use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait Agent: Debug {

    // This function will allow agents to execute their logic.
    async fn execute(&mut self, project: &mut Project) -> Result<(), Box<dyn std::error::Error>>;

}