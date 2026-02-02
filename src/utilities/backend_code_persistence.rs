/*
 *  utilities/backend_code_persistence.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/25/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use std::fs;
use std::path::{ Path, PathBuf };

const BACKEND_CODE_PROJECT_PATH: &str           = "backend_code";
const CODE_TEMPLATE_RELATIVE_FILE_PATH: &str    = "backend_code/src/template.rs";
const EXECUTABLE_MAIN_RELATIVE_PATH: &str       = "backend_code/src/main.rs";
const API_SCHEMA_RELATIVE_PATH: &str            = "backend_code/schemas/api_schema.json";

pub struct BackendCodePersistence {}

impl BackendCodePersistence {
    // Get backend code project path.
    pub fn get_backend_code_project_path() -> PathBuf {
        let project_directory = env!("CARGO_MANIFEST_DIR");
        let backend_code_project_path_string = format!("{}/{}", project_directory, BACKEND_CODE_PROJECT_PATH);
        let backend_code_project_path = Path::new(&backend_code_project_path_string);
        backend_code_project_path.to_path_buf()
    }

    // Get code template file path.
    fn get_code_template_file_path() -> PathBuf {
        let project_directory = env!("CARGO_MANIFEST_DIR");
        let file_path_string = format!("{}/{}", project_directory, CODE_TEMPLATE_RELATIVE_FILE_PATH);
        let file_path = Path::new(&file_path_string);
        file_path.to_path_buf()
    }

    // Get executable main file path.
    fn get_executable_main_file_path() -> PathBuf {
        let project_directory = env!("CARGO_MANIFEST_DIR");
        let file_path_string = format!("{}/{}", project_directory, EXECUTABLE_MAIN_RELATIVE_PATH);
        let file_path = Path::new(&file_path_string);
        file_path.to_path_buf()
    }

    // Get API schemas file path.
    fn get_api_schemas_file_path() -> PathBuf {
        let project_directory = env!("CARGO_MANIFEST_DIR");
        let file_path_string = format!("{}/{}", project_directory, API_SCHEMA_RELATIVE_PATH);
        let file_path = Path::new(&file_path_string);
        file_path.to_path_buf()
    }

    // Get code template.
    pub fn read_code_template_contents() -> String {
        let code_template_file_path = Self::get_code_template_file_path();
        let code_template = fs::read_to_string(code_template_file_path)
            .expect("Failed to read code template.");
        code_template
    }

    // Get executable main contents.
    pub fn read_executable_main_contents() -> String {
        let executable_main_file_path = Self::get_executable_main_file_path();
        let executable_main_contents = fs::read_to_string(executable_main_file_path)
            .expect("Failed to read executable main contents.");
        executable_main_contents
    }

    // Save new backend code.
    pub fn save_backend_code(contents: &String) {
        let executable_main_file_path = Self::get_executable_main_file_path();
        fs::write(executable_main_file_path, contents)
            .expect("Failed to write main.rs file.");
    }

    // Save JSON API endpoint schema.
    pub fn save_api_endpoints(api_endpoints: &String) {
        let api_schemas_file_path = Self::get_api_schemas_file_path();
        fs::write(api_schemas_file_path, api_endpoints)
            .expect("Failed to write API endpoints to file.");
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_code_paths() {
        let backend_code_project_path = BackendCodePersistence::get_backend_code_project_path();
        assert!(backend_code_project_path.exists());

        let code_template_file_path = BackendCodePersistence::get_code_template_file_path();
        assert!(code_template_file_path.exists());

        let executable_main_file_path = BackendCodePersistence::get_executable_main_file_path();
        assert!(executable_main_file_path.exists());

        let api_schemas_file_path = BackendCodePersistence::get_api_schemas_file_path();
        assert!(api_schemas_file_path.exists());
    }
}