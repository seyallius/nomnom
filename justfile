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

# Check rust code without building
[group('Rust')]
c mode="":
    cargo check

# Build Rust binary
[group('Rust')]
b mode="":
    cargo build {{mode}}

# Run Rust binary
[group('Rust')]
r mode="": b
    cargo r {{mode}}

# ------------------------------------------------------------------------------
# Code Quality
# ------------------------------------------------------------------------------

# Format Rust code
[group('Code Quality')]
fmt-rust:
    cargo fmt

# Run Rust linter (clippy)
[group('Code Quality')]
clippy:
    cargo clippy -- -D warnings

# ------------------------------------------------------------------------------
# Testing
# ------------------------------------------------------------------------------

# Run Rust tests
[group('Testing')]
test-rust:
    cargo test

# ------------------------------------------------------------------------------
# Cleanup
# ------------------------------------------------------------------------------

# Clean Rust build artifacts
[group('Cleanup')]
clean-rust:
    cargo clean

# ------------------------------------------------------------------------------
# Git
# ------------------------------------------------------------------------------

# Rebase current branch to the specified number of commits (Usage: just rebase 5)
[group('Git')]
rebase n="3":
    git rebase -i HEAD~{{n}}
