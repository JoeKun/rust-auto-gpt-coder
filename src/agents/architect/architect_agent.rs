/*
 *  agents/architect/architect_agent.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/31/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use super::architect_ai_functions::{ print_project_scope, print_site_urls };
use super::super::common::attributes::{ AgentAttributes, AgentStatus };
use super::super::common::traits::Agent;

use crate::model::{ project::Project, project_scope::ProjectScope };
use crate::utilities::ai_tasks::ai_task_request_decoded;
use crate::utilities::command_line::CommandLine;
use crate::utilities::networking::check_status_code;

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

// Solutions Architect
#[derive(Debug)]
pub struct SolutionArchitectAgent {
    pub attributes: AgentAttributes,
}

impl SolutionArchitectAgent {
    pub fn new() -> Self {
        let attributes = AgentAttributes {
            objective: "Gathers information and design solutions for website development".to_string(),
            position: "Solutions Architect".to_string(),
            status: AgentStatus::Discovery,
        };
        Self {
            attributes: attributes
        }
    }

    async fn determine_project_scope(&mut self, project: &mut Project) -> ProjectScope {
        let message_context = format!("{}", project.description);
        let ai_response: ProjectScope = ai_task_request_decoded(
            message_context, 
            &self.attributes.position, 
            get_function_string!(print_project_scope), 
            print_project_scope,
        ).await;

        project.scope = Some(ai_response.clone());
        self.attributes.status = AgentStatus::Finished;
        return ai_response;
    }

    async fn determine_external_urls(&mut self, project: &mut Project, message_context: String) {
        let ai_response: Vec<String> = ai_task_request_decoded(
            message_context, 
            &self.attributes.position, 
            get_function_string!(print_site_urls), 
            print_site_urls,
        ).await;

        project.external_urls = Some(ai_response);
        self.attributes.status = AgentStatus::UnitTesting;
    }
}

#[async_trait]
impl Agent for SolutionArchitectAgent {
    async fn execute(&mut self, project: &mut Project) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.status != AgentStatus::Finished {
            match self.attributes.status {

                AgentStatus::Discovery => {
                    let project_scope = self.determine_project_scope(project).await;
                    if project_scope.is_external_urls_required {
                        self.determine_external_urls(project, project.description.clone()).await;
                        self.attributes.status = AgentStatus::UnitTesting;
                    }
                },

                AgentStatus::UnitTesting => {
                    let mut exclude_urls: Vec<String> = vec![];
                    let client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .expect("Unable to instantiate reqwest client.");

                    // Find faulty URLs.
                    let urls = project
                        .external_urls.as_ref()
                        .expect("No URL object in project structure.");

                    for url in urls {
                        let endpoint_string = format!("Testing URL endpoint: {}", url);
                        CommandLine::print_agent_unit_test_message(
                            &self.attributes.position, 
                            endpoint_string.as_str(), 
                        );

                        // Perform URL test.
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone());
                                }
                            },
                            Err(error) => {
                                println!("Error checking {}: {}", url, error);
                            },
                        }
                    }

                    // Exclude any faulty URLs.
                    if exclude_urls.len() > 0 {
                        let new_urls = project
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(&url))
                            .cloned()
                            .collect();
                        project.external_urls = Some(new_urls);
                    }

                    // Confirm done.
                    self.attributes.status = AgentStatus::Finished;
                },

                // Default to finished status.
                _ => {
                    self.attributes.status = AgentStatus::Finished;
                }
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Disable this test for global `cargo test` commands, 
    // because it requires OpenAI keys in environment variables.
    //#[tokio::test]
    #[allow(dead_code)]
    async fn test_solution_architect() {
        let mut agent = SolutionArchitectAgent::new();
        let mut project = Project {
            description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
            scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };
        agent.execute(&mut project)
            .await
            .expect("Unable to execute Solutions Architect Agent.");
        assert!(project.scope.is_some());
        assert!(project.external_urls.is_some());

        dbg!(project);
    }
}