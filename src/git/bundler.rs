use regex::Regex;
use std::collections::HashMap;

pub fn bundle_diff(raw_diff: &str, aggressive: bool) -> String {
    let mut result = String::new();
    let mut var_map: HashMap<String, String> = HashMap::new();
    let mut var_counter = 0;

    let re_metadata = Regex::new(r"^(index|diff --git|---|\+\+\+).*").unwrap();
    let re_comments = Regex::new(r"(//.*|/\*[\s\S]*?\*/)").unwrap();
    let re_long_vars = Regex::new(r"\b[a-zA-Z_]{10,}\b").unwrap();

    for line in raw_diff.lines() {
        if re_metadata.is_match(line) { continue; }
        
        let mut processed = re_comments.replace_all(line, "").to_string();
        processed = processed.trim().to_string();

        if aggressive && processed.len() > 0 {
            processed = re_long_vars.replace_all(&processed, |caps: &regex::Captures| {
                let var = &caps[0];
                var_map.entry(var.to_string()).or_insert_with(|| {
                    var_counter += 1;
                    format!("v{}", var_counter)
                }).clone()
            }).to_string();
        }

        if !processed.is_empty() {
            result.push_str(&processed);
            result.push('\n');
        }
    }
    result
}