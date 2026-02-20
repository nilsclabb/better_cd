# Add this to your ~/.zshrc file
function cd+() {
    # Run the rust macro, capture its standard output (the selected path)
    local DEST=$(/Users/claesson/codewithnils/better_cd/target/release/better_cd "$@")
    if [ -n "$DEST" ]; then
        cd "$DEST"
    fi
}
