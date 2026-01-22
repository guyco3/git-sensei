pub fn print_hook(shell: &str) {
    match shell {
        "zsh" => println!(r#"
# GitSensei ZLE Hijack with Animation
_gitsensei_tab_complete() {{
    # Trigger only inside git commit -m or -am
    if [[ $BUFFER == *"git commit"*"-m "* || $BUFFER == *"git commit"*"-am "* ]]; then
        # 1. Show Loading State
        zle -R "Sensei is thinking..."
        
        # 2. Extract context
        local prefix="${{BUFFER#*\"}}"
        prefix="${{prefix%\"}}"

        # 3. Call binary (the terminal will show the message until this finishes)
        local suggestion=$(git-sensei suggest "$prefix" 2>/dev/null)

        if [[ -n "$suggestion" ]]; then
            # Reconstruct buffer and move cursor
            local base="${{BUFFER%\"*}}"
            BUFFER="${{base}}\"${{prefix}}${{suggestion}}\""
            CURSOR=$(($#BUFFER - 1))
            
            # Clear loading message
            zle -R ""
            return 0
        fi
    fi
    
    # Fallback to default
    zle -R ""
    zle expand-or-complete
}}

zle -N _gitsensei_tab_complete
bindkey '^I' _gitsensei_tab_complete
"#),
        _ => eprintln!("Shell not supported."),
    }
}