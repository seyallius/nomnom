set shell := ["bash", "-c"]

# Build Configurations
rust_dir := `pwd`
rust_bin := "nomnom"

# Paths
root_dir := `pwd`

# Colors (Use single quotes for raw strings to pass \033 to echo)
c_cyan := '\033[36m'
c_green := '\033[32m'
c_reset := '\033[0m'
c_bold := '\033[1m'

# Default target: List available commands
default:
    @just --list

# ------------------------------------------------------------------------------
# Rust
# ------------------------------------------------------------------------------

# Set nightly version for rust for this project
[group('Rust')]
rust-nightly:
    rustup override set nightly

# Unset the nightly rust version
[group('Rust')]
rust-nightly-unset:
    rustup override unset

# Build Rust binary (release)
[group('Rust')]
rustb:
    cd {{rust_dir}} && cargo build --release

# Run Rust binary with timing
[group('Rust')]
rust: rustb
    {{rust_bin}}

# ------------------------------------------------------------------------------
# Code Quality
# ------------------------------------------------------------------------------

# Format Rust code
[group('Code Quality')]
fmt-rust:
    cd {{rust_dir}} && cargo fmt

# Check Rust code without building
[group('Code Quality')]
check-rust:
    cd {{rust_dir}} && cargo check

# Run Rust linter (clippy)
[group('Code Quality')]
clippy:
    cd {{rust_dir}} && cargo clippy -- -D warnings

# ------------------------------------------------------------------------------
# Testing
# ------------------------------------------------------------------------------

# Run Rust tests
[group('Testing')]
test-rust:
    cd {{rust_dir}} && cargo test

# ------------------------------------------------------------------------------
# Cleanup
# ------------------------------------------------------------------------------

# Clean Rust build artifacts
[group('Cleanup')]
clean-rust:
    cd {{rust_dir}} && cargo clean

# ------------------------------------------------------------------------------
# Git
# ------------------------------------------------------------------------------

# Rebase current branch to the specified number of commits (Usage: just rebase 5)
[group('Git')]
rebase n="3":
    git rebase -i HEAD~{{n}}
