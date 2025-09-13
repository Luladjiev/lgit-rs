# lgit-rs

[![Rust](https://github.com/Luladjiev/lgit-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Luladjiev/lgit-rs/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/lgit.svg)](https://crates.io/crates/lgit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/lgit.svg)](https://crates.io/crates/lgit)

`lgit-rs` is a powerful, opinionated command-line interface (CLI) tool, designed to simplify the management of git
repositories. It provides a set of commands that streamline common git operations, making your workflow more efficient.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
- [Configuration](#configuration)
- [Why lgit?](#why-lgit)
- [Development](#development)
- [Contributing](#contributing)
- [FAQ & Troubleshooting](#faq--troubleshooting)
- [License](#license)

## Features

- **Autosquash** (`as`): Automatically squash all fixup commits in the current branch, cleaning up your commit history with interactive rebase. Perfect for consolidating work-in-progress commits.

- **Branch** (`b`): Quickly create a new branch from a freshly pulled BASE branch (defaults to main/master). Ensures you're always branching from the latest code.

- **Checkout** (`co`): Checkout a branch by name with fuzzy matching, or interactively select from a list of all local/remote branches. Supports `--remote` and `--all` flags for filtering.

- **CherryPick** (`cp`): Interactively select and cherry-pick commits from another branch using a fuzzy finder. Makes it easy to apply specific commits across branches.

- **DeleteBranches**: Safely delete all local branches whose remote tracking branches no longer exist. Helps keep your local repository clean.

- **Fixup** (`f`): Commit changes as a fixup commit that can later be automatically squashed with autosquash. Streamlines the process of fixing up previous commits.

- **Rebase** (`r`): Rebase the current branch on top of a freshly pulled BASE branch with a single command. Keeps your feature branches up to date.

- **Git Command Fallback**: For any git command not directly supported by lgit, the tool will automatically pass the command through to git, making lgit a drop-in replacement.

## Requirements

- **Git**: Version 2.0 or higher
- **Rust**: Version 1.70 or higher (if building from source)

You can check your versions:

```bash
git --version
rustc --version  # if building from source
```

## Installation

### Precompiled Binaries (Recommended)

[Download precompiled binaries for Windows, macOS and Linux](https://github.com/Luladjiev/lgit-rs/releases) from the releases page.

### Using Cargo

Installing through Cargo is the easiest way if you have Rust installed:

```bash
cargo install lgit
```

To update to the latest version:
```bash
cargo install lgit
```

### Building from Source

Clone and build the project locally:

```bash
git clone https://github.com/Luladjiev/lgit-rs.git
cd lgit-rs
cargo install --path .
```

### Troubleshooting Installation

**Cargo not found**: Install Rust and Cargo from [rustup.rs](https://rustup.rs/)

**Permission denied**: On macOS/Linux, you might need to add `~/.cargo/bin` to your PATH:
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Old version**: Make sure you're getting the latest version:
```bash
cargo install lgit --force
```

## Quick Start

Here are some common workflows to get you started with lgit:

### Creating and Working on a Feature Branch

```bash
# Create a new feature branch from latest main
lgit branch feature/awesome-feature

# Make your changes, then commit as fixup for easy cleanup later
lgit fixup

# Make more changes, create another fixup
lgit fixup

# Squash all fixup commits when you're ready
lgit autosquash

# Keep your branch up to date with main
lgit rebase
```

### Branch Management

```bash
# Checkout a branch interactively
lgit checkout

# Clean up old branches whose remotes are gone
lgit delete-branches

# Cherry-pick commits from another branch interactively
lgit cherry-pick other-branch
```

### Using Git Commands

```bash
# Any git command works through lgit
lgit status
lgit log --oneline
lgit push origin main

# Explicitly pass commands to git with --
lgit -- branch -d old-feature
```

## Usage

To get a comprehensive list of all available commands and options, you can use the `--help` flag:

```bash
lgit --help
```

Each command has a dedicated help page that can be accessed by running `lgit <command> --help`. For example:

```bash
lgit branch --help
```

### Git Command Fallback

If you run a git command that is not directly supported by lgit, the tool will automatically pass the command through to git. This means you can use lgit as a drop-in replacement for git:

```bash
# These commands will be passed through to git
lgit status
lgit log --oneline
lgit diff HEAD~1
```

You can also explicitly execute git commands by using `--` followed by the git command:

```bash
# Explicitly pass commands to git using --
lgit -- status
lgit -- log --graph --all
lgit -- reset --hard HEAD~1
```

### Command Examples

#### Autosquash
```bash
# Squash all fixup commits in current branch
lgit autosquash

# Squash last 3 commits
lgit as --number 3

# Squash commits since branching from main
lgit as --base main
```

#### Branch Operations
```bash
# Create branch from default base (main/master)
lgit branch my-feature

# Create branch from specific base
lgit b my-feature --base develop

# Interactive checkout
lgit checkout

# Checkout with fuzzy matching
lgit co my-feat  # matches "my-feature"

# List only remote branches
lgit co --remote

# List all branches (local + remote)
lgit co --all
```

#### Fixup Commits
```bash
# Create fixup commit with staged changes
lgit fixup

# Shorthand
lgit f
```

#### Cherry-pick
```bash
# Interactive cherry-pick from another branch
lgit cherry-pick feature/other-branch

# Shorthand
lgit cp main
```

#### Rebase
```bash
# Rebase current branch on freshly pulled main
lgit rebase

# Rebase on specific base branch
lgit r --base develop
```

#### Cleanup
```bash
# Delete branches whose remotes are gone
lgit delete-branches
```

## Configuration

lgit uses your existing git configuration and doesn't require additional setup. However, you can configure some behaviors:

### Default Base Branch

lgit automatically detects your main branch (main, master).

### Git Integration

lgit respects all your existing git configurations including:
- User name and email
- Remote configurations
- Git aliases
- Git hooks

## Why lgit?

lgit streamlines common git workflows by providing opinionated, high-level commands that combine multiple git operations. Here's how lgit compares to standard git commands:

| Task | Git Commands | lgit Command |
|------|-------------|--------------|
| Create branch from latest main | `git checkout main && git pull && git checkout -b feature` | `lgit branch feature` |
| Fixup and squash commits | `git add -A && git commit --fixup=HEAD~1 && git rebase -i --autosquash HEAD~3` | `lgit fixup && lgit autosquash` |
| Interactive branch checkout | `git branch -a` → copy/paste branch name → `git checkout branch` | `lgit checkout` |
| Clean up merged branches | `git branch -d branch1 && git branch -d branch2...` | `lgit delete-branches` |
| Rebase on latest main | `git checkout main && git pull && git checkout - && git rebase main` | `lgit rebase` |

### Key Benefits

- **Fewer Commands**: Complex workflows become single commands
- **Interactive Menus**: Fuzzy-finding for branches, commits, and more
- **Smart Defaults**: Automatically detects main branch, pulls latest changes
- **Safety First**: Confirmation prompts for destructive operations
- **Git Compatibility**: Drop-in replacement - all git commands still work
- **Workflow Focused**: Designed around real development workflows, not just git primitives

## Development

`lgit-rs` is developed using the [Rust programming language](https://www.rust-lang.org/) and
the [Cargo package manager](https://doc.rust-lang.org/cargo/).

You can clone the repository
and run the project locally using the following commands:

```bash
git clone https://github.com/luladjiev/lgit-rs.git
cd lgit-rs
cargo run
```

## Contributing

We welcome contributions from the community! Here's how you can help improve lgit:

### Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/lgit-rs.git
   cd lgit-rs
   ```
3. **Create a feature branch**:
   ```bash
   lgit branch feature/your-feature-name
   ```

### Development Workflow

```bash
# Install dependencies and build
cargo build

# Run tests
cargo test

# Run lgit locally during development
cargo run -- --help

# Format code (use rustfmt)
cargo fmt

# Run linting
cargo clippy

# Before submitting, run all checks
cargo test && cargo fmt && cargo clippy
```

### Submitting Changes

1. **Test your changes** thoroughly
2. **Update documentation** if needed
3. **Commit your changes** using conventional commits:
   ```bash
   lgit fixup
   lgit autosquash
   ```
4. **Push to your fork** and **create a Pull Request**

### What to Contribute

- 🐛 **Bug fixes** - Help us squash bugs!
- ✨ **New features** - Add new git workflow commands
- 📚 **Documentation** - Improve README, add examples
- 🧪 **Tests** - Increase test coverage
- 🎨 **Code quality** - Refactoring, performance improvements

### Project Structure

```
src/
├── commands/          # Individual command implementations
│   ├── autosquash.rs
│   ├── branch.rs
│   └── ...
├── cli.rs            # Command-line interface definitions
├── commands.rs       # Command dispatch logic
├── main.rs           # Application entry point
└── utils.rs          # Shared utilities
```

### Reporting Issues

Found a bug? Have a feature request? [Open an issue](https://github.com/Luladjiev/lgit-rs/issues) with:
- Clear description of the problem or feature
- Steps to reproduce (for bugs)
- Expected vs actual behavior
- Your system info (OS, git version, lgit version)

## FAQ & Troubleshooting

### Common Questions

**Q: Does lgit work with existing git repositories?**
A: Yes! lgit works with any existing git repository. It uses your current git configuration and doesn't modify your repository structure.

**Q: Can I use lgit alongside regular git commands?**
A: Absolutely. lgit is designed as a complement to git, not a replacement. You can mix lgit and git commands freely.

**Q: What happens if I run a git command that lgit doesn't support?**
A: lgit will automatically pass the command through to git, so `lgit status` works the same as `git status`.

**Q: Does lgit support git hooks?**
A: Yes, lgit respects all existing git hooks since it uses git under the hood.

### Troubleshooting

**"Command not found: lgit"**
- Make sure `~/.cargo/bin` is in your PATH
- Try running `cargo install lgit --force` to reinstall

**"Git command failed"**
- Ensure you're in a git repository: `git status`
- Check that git is working: `git --version`
- Verify you have the necessary permissions

**"No base branch found"**
- lgit looks for main or master branches
- Or specify manually: `lgit rebase --base your-branch`

**Interactive menus not working**
- Ensure you're using a compatible terminal
- Try updating to the latest version: `cargo install lgit --force`
- Check that your terminal supports interactive input

**"Branch already exists"**
- Use `lgit checkout existing-branch` to switch to existing branches
- Use `lgit branch new-branch` only for creating new branches

### Compatibility

- **Git Version**: Requires git 2.0+
- **Operating Systems**: Windows, macOS, Linux
- **Terminals**: Works with all major terminal emulators
- **Git Workflows**: Compatible with GitFlow, GitHub Flow, and custom workflows

## License

`lgit-rs` is licensed under the [MIT License](https://choosealicense.com/licenses/mit/), a permissive license that lets
you do anything with the code with proper attribution and without warranty.
