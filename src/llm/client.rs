use crate::config::Config;
use crate::git;
use crate::llm::prompt;
use serde_json::json;
use std::time::Duration;

pub fn generate_suggestion(cfg: &Config, prefix: &str) -> Result<String, Box<dyn std::error::Error>> {
    let raw_diff = git::provider::get_staged_diff();
    let files = git::provider::get_staged_files();
    let bundled = git::bundler::bundle_diff(&raw_diff, cfg.aggressive_minification);
    
    let final_prompt = prompt::build_prompt(&files, &bundled, prefix);

    let body = json!({
        "model": cfg.model,
        "prompt": final_prompt,
        "stream": false,
        "options": {
            "num_predict": 40,
            "stop": ["\n", "\"", "Assistant:"]
        }
    });

    let resp: serde_json::Value = ureq::post(&cfg.endpoint)
        .timeout(Duration::from_millis(cfg.timeout_ms))
        .send_json(body)?
        .into_json()?;

    let text = resp["response"].as_str().unwrap_or("").trim().to_string();
    Ok(text)
}