/*
 *  main.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/25/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {
        stringify!($func)
    };
}

#[macro_use]
mod api;
mod agents;
mod model;
mod utilities;

use agents::manager::managing_agent::ManagingAgent;
use utilities::command_line::CommandLine;

#[tokio::main]
async fn main() {
    let user_request = CommandLine::get_user_response("What website are we building today?");
    let mut managing_agent = ManagingAgent::new(user_request)
        .await
        .expect("Error creating managing agent");
    managing_agent.execute()
        .await;
}
