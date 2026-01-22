use crate::config::Config;
use crate::git;
use crate::llm::prompt;
use std::time::Duration;

pub fn generate_suggestion(cfg: &Config, prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
    let raw_diff = git::provider::get_staged_diff();
    let files = git::provider::get_staged_files();
    let bundled = git::bundler::bundle_diff(&raw_diff, cfg.aggressive_minification);
    
    let final_prompt = prompt::build_prompt(&cfg.system_prompt, &files, &bundled, prefix);

    // 1. Configure the Agent for 3.x
    let http_config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_millis(cfg.timeout_ms)))
        .build();
    
    let agent: ureq::Agent = http_config.into();

    let body = serde_json::json!({
        "model": &cfg.model,
        "prompt": &final_prompt,
        "stream": false,
    });

    // 2. Send the request
    // ureq 3.x send_json() works if you enable the "json" feature
    let mut response = agent.post(&cfg.endpoint)
        .header("Content-Type", "application/json")
        .send_json(&body)?;

    // 3. READ JSON (The "Secret" for 3.x)
    // You need to call .body_mut().read_json::<T>()
    // and make sure the "json" feature is in Cargo.toml
    let resp_json: serde_json::Value = response.body_mut().read_json()?;

    let text = resp_json["response"].as_str().unwrap_or("").trim().to_string();
    Ok(text)
}