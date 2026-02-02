/*
 *  utilities/networking.rs
 *  rust-auto-gpt-coder
 *
 *  Created by Joel Lopes Da Silva on 1/25/26.
 *  Copyright © 2026 Joel Lopes Da Silva. All rights reserved.
 *
 */

use reqwest::Client;

// Check whether request URL is valid.
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client
        .get(url)
        .send()
        .await?;
    Ok(response.status().as_u16())
}
