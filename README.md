# lgit-rs

`lgit-rs` is a powerful command-line interface (CLI) tool, designed to simplify the management of git
repositories. It provides a set of commands that streamline common git operations, making your workflow more efficient.

## Features

- **Branch**: Easily create a new branch from the latest BASE branch.
- **Rebase**: Rebase the current branch on top of the latest BASE branch with a single command.
- **Fixup**: Commit as a fixup, simplifying your commit history.
- **DeleteBranches**: Safely delete all branches for which remotes are gone. Use with caution!

## Requirements

### Git

`lgit-rs` requires git to be installed on your system. You can check if git is installed by running the following
command:

```bash
git --version
```

### Cargo

`lgit-rs` is developed using the Rust programming language and the Cargo package manager. You can check if Cargo is
installed by running the following command:

```bash
cargo --version
```

If Cargo is not installed, you can install it by following the instructions on
the [official website](https://www.rust-lang.org/tools/install).

## Installation

Installing `lgit-rs` is as simple as running a single command with cargo:

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

`lgit-rs` is developed using the Rust programming language and the Cargo package manager. You can clone the repository
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
