#!/bin/bash
# Setup script for publishing toon-rust to crates.io and GitHub

set -e

echo "🚀 Setting up toon-rust for publishing..."
echo ""

# Check if git is initialized
if [ ! -d .git ]; then
    echo "❌ Git not initialized. Initializing..."
    git init
fi

# Check if Cargo.toml has been updated
if grep -q "Your Name" Cargo.toml || grep -q "yourusername" Cargo.toml; then
    echo "⚠️  WARNING: Cargo.toml still contains placeholder values!"
    echo "   Please update:"
    echo "   - authors field with your name and email"
    echo "   - repository field with your GitHub repository URL"
    echo "   - homepage field with your GitHub repository URL"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Run checks
echo "📦 Running cargo checks..."
cargo check
echo "✅ cargo check passed"

echo ""
echo "🧪 Running tests..."
cargo test
echo "✅ Tests passed"

echo ""
echo "📋 Checking package contents..."
cargo package --list > /dev/null
echo "✅ Package check passed"

echo ""
echo "🔍 Running dry-run publish..."
cargo publish --dry-run
echo "✅ Dry-run passed"

echo ""
echo "✨ All checks passed! You're ready to publish."
echo ""
echo "Next steps:"
echo "1. Update Cargo.toml with your information (if not done)"
echo "2. Create GitHub repository: https://github.com/new"
echo "3. Push code: git add . && git commit -m 'Initial commit' && git push"
echo "4. Update repository URL in Cargo.toml"
echo "5. Run: cargo publish"
echo ""
echo "For detailed instructions, see PUBLISHING.md"

