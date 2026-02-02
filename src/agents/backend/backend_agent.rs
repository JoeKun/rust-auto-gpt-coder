/*
 *  agents/backend/backend_agent.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/31/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use super::backend_ai_functions::{
    print_backend_webserver_code, print_fixed_code, 
    print_improved_webserver_code, print_rest_api_endpoints, 
};
use super::super::common::attributes::{ AgentAttributes, AgentStatus };
use super::super::common::traits::Agent;

use crate::model::endpoint_route::{ EndpointRoute, HTTPMethod };
use crate::model::project::Project;
use crate::utilities::ai_tasks::ai_task_request;
use crate::utilities::backend_code_persistence::BackendCodePersistence;
use crate::utilities::command_line::CommandLine;
use crate::utilities::networking::check_status_code;

use async_trait::async_trait;
use reqwest::Client;
use std::process::{ Command, Stdio };
use std::time::Duration;
use tokio::time;

#[derive(Debug)]
pub struct BackendDeveloperAgent {
    pub attributes: AgentAttributes,
    pub bug_errors: Option<String>,
    pub bug_count: u8,
}

impl BackendDeveloperAgent {
    pub fn new() -> Self {
        let attributes = AgentAttributes {
            objective: "Develops backend code for webserver and JSON database.".to_string(),
            position: "Backend Developer".to_string(),
            status: AgentStatus::Discovery,
        };

        Self {
            attributes: attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn generate_initial_backend_code(&mut self, project: &mut Project) {
        let code_template_string = BackendCodePersistence::read_code_template_contents();

        // Concatenate instructions
        let message_context = format!(
            "CODE_TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_template_string, project.description, 
        );

        let backend_code: String = ai_task_request(
            message_context, 
            &self.attributes.position, 
            get_function_string!(print_backend_webserver_code), 
            print_backend_webserver_code,
        ).await;

        BackendCodePersistence::save_backend_code(&backend_code);
        project.backend_code = Some(backend_code);
    }

    async fn improve_backend_code(&mut self, project: &mut Project) {
        let message_context = format!(
            "CODE_TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            project.backend_code, project, 
        );

        let backend_code: String = ai_task_request(
            message_context, 
            &self.attributes.position, 
            get_function_string!(print_improved_webserver_code), 
            print_improved_webserver_code,
        ).await;

        BackendCodePersistence::save_backend_code(&backend_code);
        project.backend_code = Some(backend_code);
    }

    async fn fix_code_bugs(&mut self, project: &mut Project) {
        let message_context = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            project.backend_code, self.bug_errors, 
        );

        let backend_code: String = ai_task_request(
            message_context, 
            &self.attributes.position, 
            get_function_string!(print_fixed_code), 
            print_fixed_code,
        ).await;

        BackendCodePersistence::save_backend_code(&backend_code);
        project.backend_code = Some(backend_code);
    }

    async fn extract_rest_api_endpoints(&self) -> String {
        let backend_code = BackendCodePersistence::read_executable_main_contents();

        // Structure message context.
        let message_context = format!("CODE_INPUT: {:?}", backend_code);
        let rest_api_endpoints: String = ai_task_request(
            message_context, 
            &self.attributes.position, 
            get_function_string!(print_rest_api_endpoints), 
            print_rest_api_endpoints,
        ).await;

        println!("{}", rest_api_endpoints);

        rest_api_endpoints
    }
}

#[async_trait]
impl Agent for BackendDeveloperAgent {
    async fn execute(
        &mut self, 
        project: &mut Project
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.status != AgentStatus::Finished {
            match &self.attributes.status {

                AgentStatus::Discovery => {
                    self.generate_initial_backend_code(project).await;
                    self.attributes.status = AgentStatus::Working;
                    continue;
                },

                AgentStatus::Working => {
                    if self.bug_count == 0 {
                        self.improve_backend_code(project).await;
                    } else {
                        self.fix_code_bugs(project).await;
                    }
                    self.attributes.status = AgentStatus::UnitTesting;
                    continue;
                },

                AgentStatus::UnitTesting => {

                    // Guard: ensure AI safety.
                    CommandLine::print_agent_unit_test_message(
                        &self.attributes.position, 
                        "Backend Code Unit Testing: Requesting user input", 
                    );

                    let is_safe_code = CommandLine::confirm_safe_code();
                    if !is_safe_code {
                        panic!("Better go work on some AI alignment instead…");
                    }

                    // Build and test code.
                    CommandLine::print_agent_unit_test_message(
                        &self.attributes.position, 
                        "Backend Code Unit Testing: building project…", 
                    );

                    // Build code.
                    let build_backend_server = Command::new("cargo")
                        .arg("build")
                        .current_dir(BackendCodePersistence::get_backend_code_project_path())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("Failed to build backend application.");

                    // Determine if there are any build errors.
                    if build_backend_server.status.success() {
                        self.bug_count = 0;
                        CommandLine::print_agent_unit_test_message(
                            &self.attributes.position, 
                            "Backend Code Unit Testing: Test server build successful!", 
                        );
                    } else {
                        let error_output = build_backend_server.stderr;
                        let error_string = String::from_utf8(error_output)
                            .unwrap_or("".to_string());

                        // Update error statistics.
                        self.bug_count += 1;
                        self.bug_errors = Some(error_string);

                        // Exit if too many bugs.
                        if self.bug_count > 10 {
                            CommandLine::print_agent_error_message(
                                &self.attributes.position, 
                                "Backend Code Unit Testing: Too many bugs found in code.", 
                            );
                            panic!("Error: Too many bugs");
                        }

                        // Pass back for rework.
                        self.attributes.status = AgentStatus::Working;
                        continue;
                    }

                    // Extract API endpoints.
                    let api_endpoints_string = self.extract_rest_api_endpoints().await;
                    
                    // Convert API endpointsinto values.
                    let api_endpoints: Vec<EndpointRoute> = serde_json::from_str(&api_endpoints_string.as_str())
                        .expect("Failed to decode API endpoints.");

                    // Define endpoints to check.
                    let api_endpoints_to_check: Vec<EndpointRoute> = api_endpoints
                        .iter()
                        .filter(|&endpoint_route| {
                            endpoint_route.method == HTTPMethod::Get && endpoint_route.is_route_dynamic == false
                        })
                        .cloned()
                        .collect();

                    // Store API endpoints.
                    project.api_endpoint_schema = Some(api_endpoints_to_check.clone());

                    // Run backend application.
                    CommandLine::print_agent_unit_test_message(
                        &self.attributes.position, 
                        "Backend Code Unit Testing: Starting web server…", 
                    );

                    // Execute running server.
                    let mut run_backend_server = Command::new("cargo")
                        .arg("run")
                        .current_dir(BackendCodePersistence::get_backend_code_project_path())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to run backend application");

                    // Let user know testing on server will take place soon.
                    CommandLine::print_agent_unit_test_message(
                        &self.attributes.position, 
                        "Backend Code Unit Testing Launching tests on server in 5 seconds…", 
                    );

                    // Wait for 5 seconds.
                    let delay = Duration::from_secs(5);
                    time::sleep(delay).await;

                    // Check status code.
                    for endpoint in api_endpoints_to_check {

                        // Confirm URL testing.
                        let testing_message = format!("Testing endpoint '{}'…", endpoint.route);
                        CommandLine::print_agent_unit_test_message(
                            &self.attributes.position, 
                            testing_message.as_str(), 
                        );

                        // Create client with timeout.
                        let client = Client::builder()
                            .timeout(Duration::from_secs(5))
                            .build()
                            .expect("Unable to instantiate reqwest client.");

                        // Test URL.
                        let url = format!("http://localhost:8080{}", endpoint.route);
                        match check_status_code(&client, &url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    let error_message = format!("WARNING: Failed to call backend URL endpoint {}", endpoint.route);
                                    CommandLine::print_agent_error_message(
                                        &self.attributes.position, 
                                        error_message.as_str(), 
                                    );
                                }
                            },
                            Err(error) => {
                                // kill $(lsof -t -i:8080)
                                run_backend_server
                                    .kill()
                                    .expect("Failed to kill backend webserver.");
                                let error_message = format!("Error checking backend {}", error);
                                CommandLine::print_agent_error_message(
                                    &self.attributes.position, 
                                    error_message.as_str(), 
                                );
                            }
                        }
                    }

                    BackendCodePersistence::save_api_endpoints(&api_endpoints_string);
                    CommandLine::print_agent_unit_test_message(
                        &self.attributes.position, 
                        "Backend testing complete!", 
                    );

                    run_backend_server
                        .kill()
                        .expect("Failed to kill backend webserver on completion.");

                    self.attributes.status = AgentStatus::Finished;
                },

                AgentStatus::Finished => {},

            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Disable this test for global `cargo test` commands.
    //#[tokio::test]
    #[allow(dead_code)]
    async fn test_writing_backend_code() {
        let mut agent = BackendDeveloperAgent::new();
        let project_string = r#"
        {
            "description": "build a website which returns the current time. Use some strange library I have never heard of.",
            "scope": {
                "is_crud_required": false,
                "is_user_login_and_logout_required": false,
                "is_external_urls_required": false
            },
            "external_urls": [],
            "backend_code": null,
            "api_endpoint_schema": null
        }
        "#;

        let mut project: Project = serde_json::from_str(project_string)
            .expect("Failed to deserialize project for Backend Developer agent unit tests.");

        agent.attributes.status = AgentStatus::Discovery;
        agent.execute(&mut project)
            .await
            .expect("Failed to execute Backend Developer agent.");
        dbg!(project);
    }
}