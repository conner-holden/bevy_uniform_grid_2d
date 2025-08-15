# Justfile for bevy_uniform_grid_2d project automation

# Backport features from a source version to a target bevy branch
backport:
    #!/usr/bin/env bash
    set -euo pipefail
    
    echo "Backport Tool - Interactive Setup"
    echo "=================================="
    echo ""
    
    # Prompt for inputs
    read -p "Starting version (e.g. v0.4.1): " starting_version
    read -p "Current backport version (e.g. v0.3.0): " current_version
    read -p "Target backport version (e.g. v0.3.1): " target_version
    read -p "Bevy branch (e.g. bevy-0.15): " bevy_branch
    
    echo ""
    echo "Starting backport process..."
    echo "  From: $starting_version -> $bevy_branch branch"
    echo "  Version: $current_version -> $target_version"
    echo ""
    
    # Ensure we're starting from a clean state
    if ! git diff --quiet || ! git diff --cached --quiet; then
        echo "ERROR: Working directory is not clean. Please commit or stash changes first."
        exit 1
    fi
    
    # Checkout the target bevy branch
    echo "Checking out $bevy_branch branch..."
    git checkout $bevy_branch
    
    # Ensure we're up to date with remote
    echo "Pulling latest changes from origin/$bevy_branch..."
    git pull origin $bevy_branch || echo "WARNING: Could not pull from remote (branch may not exist yet)"
    
    # Reset to clean state (remove any uncommitted changes)
    git reset --hard HEAD
    
    # Apply all changes from the starting version
    echo "Applying changes from $starting_version..."
    git checkout $starting_version -- .
    
    # Update version in Cargo.toml to target version
    echo "Updating version to $target_version..."
    sed -i "s/^version = \".*\"/version = \"$target_version\"/" Cargo.toml
    
    # Extract bevy version from branch name (e.g., bevy-0.15 -> 0.15)
    bevy_version=$(echo "$bevy_branch" | sed 's/bevy-//')
    
    # Update Bevy dependencies to match target branch
    echo "Updating Bevy dependencies to version $bevy_version..."
    sed -i "s/bevy = { version = \"[^\"]*\"/bevy = { version = \"$bevy_version\"/" Cargo.toml
    
    echo ""
    echo "Backport preparation complete!"
    echo ""
    echo "Next steps:"
    echo "   1. Review and test the changes: cargo check"
    echo "   2. Fix any API compatibility issues for Bevy $bevy_version"
    echo "   3. Update examples if needed"
    echo "   4. Stage changes: git add ."
    echo "   5. Commit: git commit -m 'Backport $starting_version features to $target_version'"
    echo "   6. Tag: git tag $target_version"
    echo "   7. Push: git push origin $bevy_branch $target_version"
    echo ""
    echo "Current status:"
    git status --short