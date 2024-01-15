# lgit-rs

`lgit-rs` is a powerful command-line interface (CLI) tool, designed to simplify the management of git
repositories. It provides a set of commands that streamline common git operations, making your workflow more efficient.

## Features

- **Autosquash**: Automatically squash all fixup commits in the current branch.
- **Branch**: Quickly create a new branch from a freshly pulled BASE branch.
- **Checkout**:  Checkout a branch by name, or by selecting from a list of all branches.
- **DeleteBranches**: Safely delete all branches for which remotes are gone. Use with caution!
- **Fixup**: Commit as a fixup, simplifying your commit history.
- **Rebase**: Rebase the current branch on top of freshly pulled BASE branch with a single command.

## Requirements

### Git

`lgit-rs` requires git to be installed on your system. You can check if git is installed by running the following
command:

```bash
git --version
```

## Installation

[Archives of precompiled binaries for lgit are available for Windows, macOS and Linux.](https://github.com/Luladjiev/lgit-rs/releases)

### Using Cargo

Installing `lgit-rs` through Cargo is the easiest way to get started. You can install it by running the following
command:

```bash
cargo install lgit
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

We welcome contributions from the community! Feel free to submit a Pull Request or open an issue if you find any bugs or
have suggestions for improvements.

## License

`lgit-rs` is licensed under the [MIT License](https://choosealicense.com/licenses/mit/), a permissive license that lets
you do anything with the code with proper attribution and without warranty.
