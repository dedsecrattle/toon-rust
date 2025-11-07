# Quick Start: Publishing to crates.io and GitHub

## Before You Start

1. **Get a crates.io account**: https://crates.io (sign up with GitHub)
2. **Get your API token**: https://crates.io/me
3. **Login to cargo**: `cargo login <your-api-token>`

## Step-by-Step Instructions

### 1. Update Your Information

Edit `Cargo.toml` and replace:
- `Your Name <your.email@example.com>` → Your actual name and email
- `yourusername` → Your GitHub username (in repository and homepage URLs)

### 2. Create GitHub Repository

1. Go to https://github.com/new
2. Repository name: `toon-rust`
3. Description: "Token-Oriented Object Notation (TOON) - Rust implementation"
4. **Public** repository
5. **Do NOT** add README, .gitignore, or license (we have them)
6. Click "Create repository"

### 3. Push to GitHub

```bash
# Stage all files
git add .

# Create initial commit
git commit -m "Initial commit: TOON Rust implementation"

# Add your GitHub repository (replace YOUR_USERNAME)
git remote add origin https://github.com/YOUR_USERNAME/toon-rust.git

# Rename branch to main (if needed)
git branch -M main

# Push to GitHub
git push -u origin main
```

### 4. Update Cargo.toml with GitHub URL

After pushing, update `Cargo.toml` with your actual GitHub URL, then:

```bash
git add Cargo.toml
git commit -m "Update repository URL"
git push
```

### 5. Verify Everything

Run the setup script:

```bash
./setup-publish.sh
```

Or manually:

```bash
cargo check
cargo test
cargo publish --dry-run
```

### 6. Publish to crates.io

```bash
cargo publish
```

**That's it!** Your crate will be available at:
- https://crates.io/crates/toon-rust
- https://docs.rs/toon-rust

### 7. Create GitHub Release

1. Go to your repository → "Releases" → "Create a new release"
2. Tag: `v0.1.0`
3. Title: `v0.1.0 - Initial Release`
4. Click "Publish release"

## Troubleshooting

**"crate name already exists"**
- Check if `toon-rust` is available: https://crates.io/crates/toon-rust
- If taken, choose a different name in `Cargo.toml`

**"API token not found"**
- Run: `cargo login <your-api-token>`
- Get token from: https://crates.io/me

**"Repository URL invalid"**
- Make sure repository exists and is public
- Verify URL in `Cargo.toml` matches your GitHub repo

## Need Help?

See `PUBLISHING.md` for detailed instructions.

