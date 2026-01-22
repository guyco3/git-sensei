pub fn build_prompt(system_prompt: &str, files: &[String], diff: &str, prefix: &str) -> String {
    format!(
        "### Instruction: {}\n\
         ### Context: Files: {}\n\
         ### Diff:\n{}\n\n\
         ### Task: Complete the commit message.\n\
         ### Current Message: \"{}\"\n\
         ### Completion:",
        system_prompt,
        files.join(", "),
        diff,
        prefix
    )
}