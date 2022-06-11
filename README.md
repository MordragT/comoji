[![gitmoji badge](https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67.svg?style=flat-square)](https://github.com/carloscuesta/gitmoji)
![crates.io](https://img.shields.io/crates/v/gitmoji.svg)

# comoji
> A conventional commits CLI using emojis.

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
            overlays = [ comoji.overlay ];
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
Thomas Wehmöller <contact.mordrag@gmail.com>:Jonas Geschke <github@yonny.de>
Interactive git commit command line interface

USAGE:
    comoji [verbose] <SUBCOMMAND>

ARGS:
    <verbose>    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    commit    Interactively commit using the prompts
    config    Setup preferences
    help      Print this message or the help of the given subcommand(s)
    list      List all available comojis
```

### Commit

You can use the commit functionality in two ways, directly or via a commit-hook.

#### Client

Start the interactive commit client, to auto generate your commit based on your prompts.

```bash
$ gitmoji commit
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
