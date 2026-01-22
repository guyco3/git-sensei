pub fn print_hook(shell: &str) {
    match shell {
        "zsh" => println!(r#"
# GitSensei ZLE Hijack
_gitsensei_tab_complete() {{
    # Only trigger if we are inside a git commit -m or -am command
    if [[ $BUFFER == *"git commit"*"-m "* || $BUFFER == *"git commit"*"-am "* ]]; then
        # Extract everything after the first quote
        local prefix="${{BUFFER#*\"}}"
        # Remove any trailing quote if the user already typed one
        prefix="${{prefix%\"}}"

        # Call the binary
        local suggestion=$(git-sensei suggest "$prefix" 2>/dev/null)

        if [[ -n "$suggestion" ]]; then
            # Reconstruct the buffer: everything before the first quote + " + prefix + suggestion
            local base="${{BUFFER%\"*}}"
            BUFFER="${{base}}\"${{prefix}}${{suggestion}}\""
            # Move cursor to just before the closing quote
            CURSOR=$(($#BUFFER - 1))
            return 0
        fi
    fi
    # If not in a commit or AI fails, do the normal Zsh Tab behavior
    zle expand-or-complete
}}

# Bind to Tab (^I)
zle -N _gitsensei_tab_complete
bindkey '^I' _gitsensei_tab_complete
"#),
        _ => eprintln!("Shell {} not supported yet.", shell),
    }
}