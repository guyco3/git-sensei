pub fn build_prompt(system_prompt: &str, files: &[String], diff: &str, prefix: &str) -> String {
    format!(
        "{}
Examples:
Diff: + added login logic
Completion: feat: implement user login
Diff: - fixed typo in header
Completion: fix: correct spelling error in header

Context:
Files: {}
Diff:
{}

Task: Complete the commit message. 
Current: \"{}\"
Completion:",
        system_prompt,
        files.join(", "),
        diff,
        prefix
    )
}