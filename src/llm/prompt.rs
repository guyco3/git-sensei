pub fn build_prompt(system_prompt: &str, files: &[String], diff: &str, prefix: &str) -> String {
    format!(
r#"<|system|>
{system_prompt}
RULES:
- Focus ONLY on the code in the DIFF.
- Use imperative mood (e.g., "fix", "add", "refactor").
- Do not repeat the prefix.
- Output ONLY the completion text.

<|user|>
Example 1:
Files: src/math.rs
Diff: + return a + b;
Message: feat: implement addition logic

Example 2:
Files: .gitignore
Diff: + target/
Message: chore: ignore build artifacts

Current Task:
Files: {files}
Diff: 
---
{diff}
---
Complete the message: {prefix}"#,
        system_prompt = system_prompt,
        files = files.join(", "),
        diff = diff,
        prefix = prefix
    )
}