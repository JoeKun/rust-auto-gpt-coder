/*
 *  agents/manager/managing_agent.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/31/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use super::managing_ai_functions::convert_user_input_to_goal;
use super::super::common::attributes::{ AgentAttributes, AgentStatus };
use super::super::common::traits::Agent;
use super::super::architect::architect_agent::SolutionArchitectAgent;
use super::super::backend::backend_agent::BackendDeveloperAgent;

use crate::model::project::Project;
use crate::utilities::ai_tasks::ai_task_request;

#[derive(Debug)]
pub struct ManagingAgent {
    pub attributes: AgentAttributes,
    pub project: Project,
    pub agents: Vec<Box<dyn Agent>>,
}

impl ManagingAgent {
    pub async fn new(user_request: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position = "Project Manager".to_string();
        let attributes = AgentAttributes {
            objective: "Manage agents who are building an excellent website for the user".to_string(),
            position: position.clone(),
            status: AgentStatus::Discovery,
        };

        let description: String = ai_task_request(
            user_request, 
            &position, 
            get_function_string!(convert_user_input_to_goal), 
            convert_user_input_to_goal,
        ).await;

        let agents: Vec<Box<dyn Agent>> = vec![];
        let project = Project {
            description,
            scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        Ok(Self {
            attributes, 
            project,
            agents,
        })
    }

    fn add_agent(&mut self, agent: Box<dyn Agent>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(SolutionArchitectAgent::new()));
        self.add_agent(Box::new(BackendDeveloperAgent::new()));
    }

    pub async fn execute(&mut self) {
        self.create_agents();
        for agent in &mut self.agents {
            let agent_result = agent.execute(&mut self.project)
                .await;
            _ = agent_result;
        }
        _ = self.attributes.objective;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // Disable this test for global `cargo test` commands.
    //#[tokio::test]
    #[allow(dead_code)]
    async fn test_managing_agent() {
        let user_request = "need a full stack app that fetches and tracks my fitness progress. Needs to include timezone info from the web.";
        let mut managing_agent = ManagingAgent::new(user_request.to_string())
            .await
            .expect("Error creating managing agent.");
        managing_agent.execute()
            .await;
        dbg!(managing_agent.project);
    }
}