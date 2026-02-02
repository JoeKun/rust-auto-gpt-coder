/*
 *  agents/common/attributes.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/31/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

#[derive(Debug, PartialEq)]
 pub enum AgentStatus {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

#[derive(Debug)]
pub struct AgentAttributes {
    pub objective: String,
    pub position: String,
    pub status: AgentStatus,
}
