pub fn print_hook(shell: &str) {
    match shell {
        "zsh" => println!(r#"
_gitsensei_tab() {{
    if [[ $BUFFER == *"git commit"*"-m "* ]]; then
        zle -R "Sensei is thinking..."
        # Extract context between quotes
        local prefix="${{BUFFER#*\"}}"
        prefix="${{prefix%\"}}"
        
        local suggestion=$(git-sensei suggest "$prefix" 2>/dev/null)
        
        if [[ -n "$suggestion" ]]; then
            # Rebuild: everything before first quote + " + prefix + suggestion + "
            local base="${{BUFFER%\"*}}"
            BUFFER="${{base}}\"${{prefix}}${{suggestion}}\""
            # Position cursor right before the closing quote
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