pub fn print_hook(shell: &str) {
    match shell {
        "zsh" => println!(r#"
_gitsensei_tab() {{
    if [[ $BUFFER == *"git commit"*"-m "* ]]; then
        zle -R "Sensei is thinking..."
        local prefix="${{BUFFER#*\"}}"
        prefix="${{prefix%\"}}"
        
        # We run without --debug to keep the string clean for the buffer
        local suggestion=$(git-sensei suggest "$prefix" 2>/dev/null)
        
        if [[ -n "$suggestion" ]]; then
            local base="${{BUFFER%\"*}}"
            BUFFER="${{base}}\"${{prefix}}${{suggestion}}\""
            CURSOR=$(($#BUFFER - 1))
        fi
        zle -R ""
        return 0
    fi
    zle expand-or-complete
}}
zle -N _gitsensei_tab
bindkey '^I' _gitsensei_tab
"#),
        _ => println!(""),
    }
}