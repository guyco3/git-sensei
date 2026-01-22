#!/bin/bash

# 1. Kill any hung git-sensei processes
killall git-sensei 2>/dev/null

# 2. Build and Install
echo "🦀 Building and installing git-sensei..."
cargo install --path . --force

# 3. Clean Zsh Completion Cache (The 'Ghost' that breaks Tab)
echo "🧹 Cleaning Zsh cache..."
rm -f ~/.zcompdump*

# 4. Update .zshrc if needed
# (Checking if the hook is already there to avoid duplicates)
if ! grep -q "git-sensei hook zsh" ~/.zshrc; then
    echo "Writing hook to .zshrc..."
    echo 'eval "$(git-sensei hook zsh)"' >> ~/.zshrc
fi

# 5. Apply changes to the CURRENT session
echo "⚡ Refreshing shell..."
# We use 'exec zsh' because 'source' doesn't always clear ZLE widgets properly
exec zsh