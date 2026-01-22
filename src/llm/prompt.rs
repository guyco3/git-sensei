pub fn build_prompt(system_prompt: &str, files: &[String], diff: &str, prefix: &str) -> String {
    format!(
r#"<|system|>
{system_prompt}
CRITICAL: 
- Lines starting with '-' were DELETED.
- Lines starting with '+' were ADDED.
- Focus on the ACTION (e.g., "remove", "add", "refactor").

<|user|>
### Example Deletion:
Diff: 
---
- println!("debug logs");
---
Message: chore: remove debug logging

### Example Addition:
Diff: 
---
+ use std::time::Duration;
---
Message: chore: import Duration for timeouts

### Actual Task:
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