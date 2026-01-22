pub fn build_prompt(system_prompt: &str, files: &[String], diff: &str, prefix: &str) -> String {
    format!(
r#"<|system|>
{system_prompt}
RULES:
- Use imperative mood ("add", not "added").
- Refer to specific lines in the diff (e.g., "contributing doc").
- Do NOT repeat the prefix if it's already provided.
- Do NOT provide explanations, intros, or markdown blocks.
- STOP immediately after completing the sentence.

<|user|>
### Examples for Style:
Files: README.md
Diff: + to contribute, look at the contributing doc!
Message: docs: add link to contributing guide in README

Files: Cargo.toml
Diff: - version = "0.1.0" 
      + version = "0.1.1"
Message: chore: bump version to 0.1.1

### Task:
Files: {files}
Diff: 
---
{diff}
---

### Expected Output:
Complete the message starting with: "{prefix}"
Completion: {prefix}"#,
        system_prompt = system_prompt,
        files = files.join(", "),
        diff = diff,
        prefix = prefix
    )
}