use std::process::Command;

pub fn get_staged_diff() -> String {
    let output = Command::new("git")
        .args(["diff", "--cached", "-U0", "--minimal"])
        .output()
        .expect("Git diff failed");
    
    String::from_utf8_lossy(&output.stdout).to_string()
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