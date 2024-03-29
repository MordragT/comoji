//! comoji is a command line utility to handle organized commit messages by leveraging conventional commits.
//! But instead of an textual representation, emojis are used.
//! It contains an interactive commit interface to handle commit data:
//! * Emoji
//! * Title
//! * Message
//! * Optional Scope
//! * Optional Issue
//! * Optional Signed Commit
//!
//! # Installation
//! With cargo ```cargo install --path . --locked```
//! # Usage
//! ## List
//! You can just list all emojis which can be used with ```comoji list```.
//! ## Configure
//! You can configure optional prompts or defaults by executing ```comoji config```.
//! ## Commit
//! To start the interactive commit interface type ```comoji commit```.

use crate::{commit::*, config::*, emoji::EMOJIS};
use clap::{Parser, Subcommand};
use dialoguer::theme::ColorfulTheme;
use directories::BaseDirs;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use miette::{diagnostic, IntoDiagnostic, Result};
use std::fs;

pub mod commit;
pub mod config;
pub mod emoji;
pub mod error;

/// Interactive git commit command line interface
#[derive(Parser, Debug)]
#[command(author, about)]
struct App {
    #[clap(subcommand)]
    command: Cmd,
}

#[derive(Subcommand, Debug)]
#[command(author, about)]
enum Cmd {
    /// List all available comojis
    List,
    /// Interactively commit using the prompts-
    Commit(Config),
    /// Setup preferences
    Config,
}
fn main() -> Result<()> {
    let app = App::parse();

    let config_path = match BaseDirs::new() {
        Some(dirs) => dirs.config_dir().join("comoji/config.toml"),
        None => return Err(diagnostic!("Could not find config directory.").into()),
    };
    let theme = ColorfulTheme::default();

    match app.command {
        Cmd::List => {
            for comoji in EMOJIS {
                println!("{comoji}");
            }
            Ok(())
        }
        Cmd::Commit(config) => {
            let config: Config = Figment::new()
                .merge(Serialized::defaults(config))
                .merge(Env::prefixed("COMOJI_"))
                .merge(Toml::file(config_path))
                .extract()
                .into_diagnostic()?;

            let commit = Commit::prompt(&theme, &config)?;
            commit.build()?;

            Ok(())
        }
        Cmd::Config => {
            let config = Config::prompt(&theme)?;
            let config_toml = toml::to_string(&config).into_diagnostic()?;
            fs::write(config_path, config_toml).into_diagnostic()?;

            Ok(())
        }
    }
}
