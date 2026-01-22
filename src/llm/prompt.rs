pub fn build_prompt(system_prompt: &str, files: &[String], diff: &str, prefix: &str) -> String {
    format!(
        "<|im_start|>system\n{}\n<|im_end|>\n\
        <|im_start|>user\nFiles: {}\nDiff:\n{}\nPrefix: {}\n<|im_end|>\n\
        <|im_start|>assistant\n{}",
        system_prompt,
        files.join(", "),
        diff,
        prefix,
        prefix
    )
}