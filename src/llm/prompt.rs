pub fn build_prompt(_system_prompt: &str, files: &[String], diff: &str, prefix: &str) -> String {
    format!(
        "Task: Write a tiny git commit message.
Constraint: Return ONLY the message. No intro. No explanation.
Files: {}
Diff:
{}

Commit Message: {}",
        files.join(", "),
        diff,
        prefix
    )
}