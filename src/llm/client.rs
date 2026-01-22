use crate::config::Config;
use crate::git;
use crate::llm::prompt;
use std::time::Duration;

pub fn generate_suggestion(cfg: &Config, prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
    let raw_diff = git::provider::get_staged_diff();
    
    if raw_diff.trim().is_empty() {
        return Ok("No staged changes found.".to_string());
    }

    let files = git::provider::get_staged_files();
    let bundled = git::bundler::bundle_diff(&raw_diff, cfg.aggressive_minification);
    
    let final_prompt = prompt::build_prompt(&cfg.system_prompt, &files, &bundled, prefix);

    let client_config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_millis(cfg.timeout_ms)))
        .build();
    
    let agent: ureq::Agent = client_config.into();

    let body = serde_json::json!({
        "model": &cfg.model,
        "prompt": &final_prompt,
        "stream": false,
        "options": {
            "num_predict": 32,
            "temperature": 0.1,
            "stop": ["\n", "###", "\""]
        }
    });

    let mut response = agent.post(&cfg.endpoint)
        .header("Content-Type", "application/json")
        .send_json(&body)?;

    let resp_json: serde_json::Value = response.body_mut().read_json()?;
    
    let text = resp_json.get("response")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    if text.is_empty() {
        return Err("AI returned an empty response".into());
    }

    // Clean up prefix overlap
    let mut cleaned = text.replace("git commit -m", "").trim_matches(|c| c == '"' || c == '\'').to_string();
    if !prefix.is_empty() && cleaned.to_lowercase().starts_with(&prefix.to_lowercase()) {
        cleaned = cleaned[prefix.len()..].trim().to_string();
    }

    Ok(cleaned)
}