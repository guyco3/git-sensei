pub fn print_hook(shell: &str) {
    match shell {
        "zsh" => println!(r#"
_gitsensei_suggest() {{
    local prefix="${{words[CURRENT]}}"
    local suggestion=$(gitsensei suggest "$prefix")
    if [[ -n "$suggestion" ]]; then
        compadd -U "$suggestion"
    fi
}}
compdef _gitsensei_suggest gitsensei
"#),
        _ => eprintln!("Shell {} not supported yet.", shell),
    }
}