# Publishing Guide

This guide will help you publish `toon-rust` to crates.io and set up a GitHub repository.

## Prerequisites

1. **crates.io account**: Sign up at https://crates.io
2. **GitHub account**: Sign up at https://github.com
3. **cargo login**: Run `cargo login <your-api-token>` (get token from https://crates.io/me)

## Step 1: Update Cargo.toml

Before publishing, update `Cargo.toml` with your information:

1. Replace `authors = ["Your Name <your.email@example.com>"]` with your actual name and email
2. Replace `repository = "https://github.com/yourusername/toon-rust"` with your actual GitHub repository URL

## Step 2: Create GitHub Repository

1. Go to https://github.com/new
2. Repository name: `toon-rust` (or your preferred name)
3. Description: "Token-Oriented Object Notation (TOON) - Rust implementation"
4. Set to **Public**
5. **Do NOT** initialize with README, .gitignore, or license (we already have these)
6. Click "Create repository"

## Step 3: Push to GitHub

```bash
# Add all files
git add .

# Create initial commit
git commit -m "Initial commit: TOON Rust implementation"

# Add your GitHub repository as remote (replace YOUR_USERNAME)
git remote add origin https://github.com/YOUR_USERNAME/toon-rust.git

# Push to GitHub
git branch -M main
git push -u origin main
```

## Step 4: Update Cargo.toml with GitHub URL

After pushing to GitHub, update the `repository` field in `Cargo.toml` with your actual GitHub URL, then commit:

```bash
git add Cargo.toml
git commit -m "Update repository URL"
git push
```

## Step 5: Verify Before Publishing

Run these checks:

```bash
# Check for common issues
cargo check
cargo test

# Verify package metadata
cargo package --list

# Dry run (simulates publishing without actually publishing)
cargo publish --dry-run
```

Fix any errors that appear.

## Step 6: Publish to crates.io

Once everything passes:

```bash
# Publish to crates.io
cargo publish
```

**Note**: Publishing is permanent! You cannot delete or overwrite a published version. You can only publish new versions.

## Step 7: Create a Release on GitHub

1. Go to your repository on GitHub
2. Click "Releases" → "Create a new release"
3. Tag: `v0.1.0`
4. Title: `v0.1.0 - Initial Release`
5. Description: Copy from the changelog or describe the release
6. Click "Publish release"

## Versioning

For future releases:

1. Update version in `Cargo.toml` (e.g., `0.1.0` → `0.1.1` or `0.2.0`)
2. Update CHANGELOG.md (if you create one)
3. Commit changes: `git commit -am "Bump version to X.Y.Z"`
4. Tag the release: `git tag vX.Y.Z`
5. Push tags: `git push && git push --tags`
6. Publish: `cargo publish`

## Troubleshooting

### "crate name already exists"
- The name `toon-rust` might be taken. Check at https://crates.io/crates/toon-rust
- If taken, choose a different name and update `Cargo.toml`

### "API token not found"
- Run `cargo login <your-api-token>`
- Get token from https://crates.io/me

### "Repository URL invalid"
- Make sure the repository exists and is public
- Verify the URL in `Cargo.toml` matches your GitHub repository

## Additional Resources

- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Semantic Versioning](https://semver.org/)
- [crates.io Policies](https://crates.io/policies)

