<div align=center>

# comoji 🦀

[![Rust](https://img.shields.io/badge/Made_for-Rust-orange.svg?logo=rust&style=for-the-badge)](https://www.rust-lang.org/)![License](https://img.shields.io/github/license/mordragt/comoji?style=for-the-badge)

Comoji - Conventional Commits using Emojis

</div>

## About

This project was originally written for using [**gitmoji**](https://github.com/carloscuesta/gitmoji) from your command line,
but has since evolved into a helper for [**conventional commits**](https://www.conventionalcommits.org/en/v1.0.0/).
Instead of using text though, emojis are used as the commit type.

## Install

### Cargo

```bash
git clone git@github.com:MordragT/comoji.git
cd comoji
$ cargo install --path . --locked
```
### Nix Flakes

Add comoji as nix input and simply use the overlay.

```nix
{
    inputs.comoji.url = "github:MordragT/comoji";
    outputs = { self, nixpkgs, comoji, ... }@inputs:
    let
        pkgs = import nixpkgs {
            inherit system;
            overlays = [ comoji.overlays.default ];
        };
    in {
        ...
        home.packages = with pkgs; [ comoji ];
    }
}
```

## Usage

```bash
$ comoji --help
```

```
CLI for conventional emoji commits

Usage: comoji [OPTIONS] <COMMAND>

Commands:
  list    List all available comojis
  commit  Interactively commit using the prompts
  config  Setup preferences
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  
  -h, --help     Print help
```

### Commit

You can use the commit functionality in two ways, directly or via a commit-hook.

#### Client

Start the interactive commit client, to auto generate your commit based on your prompts.

```bash
$ comoji commit
```

#### Hook

Set config as shown below.

```bash
$ git add .
$ git commit
```

⚠️ The hook **should not be used** with the `comoji commit` command.

### List

Pretty print all the available comojis.

```bash
$ comoji list
```

### Config

Run `comoji config` to setup some preferences, such as the auto `git add .` feature.

## Acknowledgements

- [GitMoji](https://github.com/kegesch/gitmoji-cli) - the project comoji is built upon