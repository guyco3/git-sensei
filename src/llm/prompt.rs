pub fn build_prompt(files: &[String], diff: &str, prefix: &str) -> String {
    format!(
        "<|im_start|>system\nYou are a git commit assistant. Use Conventional Commits. Imperative mood. Max 72 chars. Return ONLY the message.\n<|im_end|>\n\
        <|im_start|>user\nFiles: {}\nDiff:\n{}\nPrefix: {}\n<|im_end|>\n\
        <|im_start|>assistant\n{}",
        files.join(", "),
        diff,
        prefix,
        prefix
    )
}