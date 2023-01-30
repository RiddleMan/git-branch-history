# git-branch-history
Tool for navigating through a history of branch checkouts.

## Installation

```shell
cargo install git-branch-history
```

## Usage

Main command:
```
$ git branch-history --help
Tool for navigating through a history of branch checkouts.

Usage: git-branch-history <COMMAND>

Commands:
  list        List history of branch checkouts
  pop-branch  Navigate to previous branch from history
  checkout    Checkout a branch from history
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

Go to previous branch alias:

```
$ git popb
```

## Features

* Navigating to the last branch
* Listing a history of branch checkouts
* Checkout of nth branch in history interactively or normally

## License

`git-branch-history` is dual-licensed under the terms of the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for details.
