use std::process::Command;

pub fn get_staged_diff() -> String {
    let output = Command::new("git")
        .args(["diff", "--cached", "--no-color"])
        .output()
        .expect("Failed to execute git diff");
    
    let result = String::from_utf8_lossy(&output.stdout).to_string();
    
    // DEBUG PRINT (only while we fix this)
    if result.is_empty() {
        eprintln!("DEBUG: Git returned an empty diff!");
    }
    
    result
}

pub fn get_staged_files() -> Vec<String> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()
        .ok();
    
    match output {
        Some(o) => String::from_utf8_lossy(&o.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect(),
        None => vec![],
    }
}