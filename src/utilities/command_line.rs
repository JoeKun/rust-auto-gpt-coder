/*
 *  utilities/command_line.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/25/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use crossterm::style::{ Color, ResetColor, SetForegroundColor };
use crossterm::ExecutableCommand;
use std::io::{ stdin, stdout };

#[derive(PartialEq, Debug)]
enum CommandLineStatementKind {
    Generation,
    UnitTest,
    Issue,
}

pub struct CommandLine {}

impl CommandLine {
    pub fn print_agent_generation_message(
        agent_position: &String, 
        agent_statement: &str, 
    ) {
        Self::print_agent_message(
            agent_position, 
            agent_statement, 
            CommandLineStatementKind::Generation, 
        );
    }

    pub fn print_agent_unit_test_message(
        agent_position: &String, 
        agent_statement: &str, 
    ) {
        Self::print_agent_message(
            agent_position, 
            agent_statement, 
            CommandLineStatementKind::UnitTest, 
        );
    }

    pub fn print_agent_error_message(
        agent_position: &String, 
        agent_statement: &str, 
    ) {
        Self::print_agent_message(
            agent_position, 
            agent_statement, 
            CommandLineStatementKind::Issue, 
        );
    }

    fn print_agent_message(
        agent_position: &String, 
        agent_statement: &str,
        agent_statement_kind: CommandLineStatementKind, 
    ) {
        let mut stdout = stdout();

        // Decide on the print color.
        let statement_color = match agent_statement_kind {
            CommandLineStatementKind::Generation    => Color::Cyan,
            CommandLineStatementKind::UnitTest      => Color::Magenta,
            CommandLineStatementKind::Issue         => Color::Red,
        };

        // Print the agent position.
        stdout.execute(SetForegroundColor(Color::Green))
            .expect("Unable to set foreground color for printing agent statement in the command line.");
        print!("Agent: {}: ", agent_position);

        // Print the agent statement.
        stdout.execute(SetForegroundColor(statement_color))
            .expect("Unable to set foreground color for printing agent statement in the command line.");
        println!("{}", agent_statement);

        // Reset color.
        stdout.execute(ResetColor)
            .expect("Unable to reset color for printing agent statement in the command line.");

    }

    pub fn get_user_response(question: &str) -> String {
        let mut stdout = stdout();

        // Print the question in a specific color.
        stdout.execute(SetForegroundColor(Color::Blue))
            .expect("Unable to set foreground color for getting user input from the command line.");
        println!("");
        println!("{}", question);

        // Reset color.
        stdout.execute(ResetColor)
            .expect("Unable to reset color for getting user input from the command line.");

        // Read user input.
        let mut user_response = String::new();
        stdin()
            .read_line(&mut user_response)
            .expect("Failed to read user response");

        // Trim whitespace and return.
        return user_response.trim().to_string();
    }

    // Get user response that code is safe to execute.
    pub fn confirm_safe_code() -> bool {
        let mut stdout = stdout();
        loop {

            // Print the question in specified color
            stdout.execute(SetForegroundColor(Color::Blue))
                .expect("Unable to set foreground color for confirming whether generated code is safe to execute.");
            println!("");
            print!("WARNING: You are about to run code written entirely by AI. ");
            println!("Review your code and confirm you wish to continue.");

            // Reset color.
            stdout.execute(ResetColor)
                .expect("Unable to reset color for confirming whether generated code is safe to execute.");

            // Present options with different colors.
            stdout.execute(SetForegroundColor(Color::Green))
                .expect("Unable to set foreground color for confirming whether generated code is safe to execute.");
            println!("[1] All good");
            stdout.execute(SetForegroundColor(Color::DarkRed))
                .expect("Unable to set foreground color for confirming whether generated code is safe to execute.");
            println!("[2] Let’s stop this project");

            // Reset color.
            stdout.execute(ResetColor)
                .expect("Unable to reset color for confirming whether generated code is safe to execute.");

            // Read user input.
            let mut user_response = String::new();
            stdin()
                .read_line(&mut user_response)
                .expect("Failed to read response");

            // Trim whitespace and convert to lowercase.
            let sanitized_user_response = user_response.trim().to_lowercase();

            // Match response.
            match sanitized_user_response.as_str() {
                "1" | "ok" | "y" => return true,
                "2" | "no" | "n" => return false,
                _ => {
                    println!("Invalid input. Please select '1' or '2'.");
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_agent_message() {
        let agent_position = "Managing Agent".to_string();
        CommandLine::print_agent_unit_test_message(
            &agent_position, 
            "Testing testing, processing something", 
        );
    }
}