pub fn build_prompt(_system: &str, files: &[String], diff: &str, prefix: &str) -> String {
    format!(
        "USER: You are a professional developer.
TASK: Write a highly specific git commit message based on the DIFF below.
FILES: {files}
DIFF:
{diff}

RULES:
1. Focus on the ACTUAL content added or removed.
2. If a specific line like 'contribute docs' was added, mention it.
3. Use Conventional Commits (feat:, fix:, docs:, style:, refactor:).
4. Be concise but descriptive.

MESSAGE START: {prefix}",
        files = files.join(", "),
        diff = diff,
        prefix = prefix
    )
}