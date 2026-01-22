use crate::config::Config;
use crate::git;
use crate::llm::prompt;
use std::time::Duration;

pub fn generate_suggestion(cfg: &Config, prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
    let raw_diff = git::provider::get_staged_diff();
    let files = git::provider::get_staged_files();
    let bundled = git::bundler::bundle_diff(&raw_diff, cfg.aggressive_minification);
    
    // Safety check: if diff is empty, don't even call the LLM
    if bundled.trim().is_empty() {
        return Ok("".to_string());
    }

    let final_prompt = prompt::build_prompt(&cfg.system_prompt, &files, &bundled, prefix);

    let config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_millis(cfg.timeout_ms)))
        .build();
    
    let agent: ureq::Agent = config.into();

    let body = serde_json::json!({
        "model": &cfg.model,
        "prompt": &final_prompt,
        "stream": false,
        "options": {
            "num_predict": 24, // Shortened even more for instant feels
            "temperature": 0.1,
            "top_k": 20,       // Narrower search is faster
            "stop": ["\n", "\"", "###"]
        }
    });

    let mut response = agent.post(&cfg.endpoint)
        .header("Content-Type", "application/json")
        .send_json(&body)?;

    let resp_json: serde_json::Value = response.body_mut().read_json()?;
    let raw_text = resp_json["response"].as_str().unwrap_or("").trim();

    // --- SAFETY PARSING ---
    // Remove "git commit -m", quotes, and backticks
    let mut cleaned = raw_text
        .replace("git commit -m", "")
        .replace("git commit", "")
        .trim_matches(|c| c == '"' || c == '\'' || c == '`')
        .to_string();

    // If prefix is "fix(ui): ", and AI says "fix(ui): resolve button alignment"
    // we only want to return "resolve button alignment" so the hook can append it.
    if !prefix.is_empty() && cleaned.to_lowercase().starts_with(&prefix.to_lowercase()) {
        cleaned = cleaned[prefix.len()..].trim().to_string();
    }

    Ok(cleaned.trim().to_string())
}