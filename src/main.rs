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
//! ## Lis
//! You can just list all emojis which can be used with ```comoji list```.
//! ## Configure
//! You can configure optional prompts or defaults by executing ```comoji config```.
//! ## Commit
//! To start the interactive commit interface type ```comoji commit```.
#[macro_use]
extern crate serde_derive;

use crate::configuration::{Configuration, EmojiFormat};
use crate::prompts::{ask_for_emoji, ask_for_issue, ask_for_message, ask_for_scope, ask_for_title};
use clap::{Arg, Command as ClapCommand};
use emoji::EMOJIS;
use error::*;
use std::process::Command;

pub mod configuration;
pub mod emoji;
pub mod error;
pub mod prompts;

fn main() {
    let matches = ClapCommand::new("comoji")
        .about("Interactive git commit command line interface")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .subcommand_required(true)
        .arg(Arg::new("verbose"))
        .subcommands([
            ClapCommand::new("list").about("List all available comojis"),
            ClapCommand::new("commit").about("Interactively commit using the prompts"),
            ClapCommand::new("config").about("Setup preferences"),
        ])
        .get_matches();

    let verbose = matches.is_present("verbose");

    match matches.subcommand() {
        Some(("list", _)) => {
            for comoji in EMOJIS {
                println!("{comoji}");
            }
        }
        Some(("commit", _)) => {
            let commit = commit();
            if commit.is_err() {
                eprintln!("Could not commit.");
                if verbose {
                    eprintln!("{:?}\n", commit);
                }
            }
        }
        Some(("config", _)) => {
            let config = config();
            if config.is_err() {
                eprintln!("Could not configure: {:?}", config);
                if verbose {
                    eprintln!("{:?}\n", config);
                }
            }
        }
        _ => unreachable!(),
    }
}

/// configures the cli
fn config() -> ComojiResult<()> {
    let mut configuration = Configuration::load()?;
    configuration.prompt()?;
    configuration.store()?;
    Ok(())
}

/// starts the interactive commit interface
fn commit() -> ComojiResult<()> {
    let emoji = ask_for_emoji(&EMOJIS)?;
    let mut scope = String::new();
    if Configuration::is_scope_prompt()? {
        scope = ask_for_scope()?;
    }
    let title = ask_for_title()?;
    let message = ask_for_message()?;

    let mut commit_title = String::new();
    if Configuration::emoji_format()? == EmojiFormat::CODE {
        commit_title += emoji.code;
    } else {
        commit_title += emoji.emoji;
    }
    commit_title += " ";
    if Configuration::is_scope_prompt()? {
        if !scope.is_empty() {
            commit_title += scope.as_str();
            commit_title += ": ";
        }
    }
    commit_title += title.as_str();

    if Configuration::is_issue_prompt()? {
        let issue = ask_for_issue()?;
        if !issue.is_empty() {
            commit_title += " (";
            commit_title += issue.as_str();
            commit_title += ")";
        }
    }

    if Configuration::is_auto_add()? {
        Command::new("git").arg("add").arg(".").output()?;
    }

    if Configuration::is_signed_commit()? {
        let git_output = Command::new("git")
            .arg("commit")
            .arg("-S")
            .arg("-m")
            .arg(commit_title)
            .arg("-m")
            .arg(message)
            .output()?;

        if git_output.status.success() {
            println!("{}", String::from_utf8_lossy(git_output.stdout.as_ref()));
        } else {
            eprintln!("{}", String::from_utf8_lossy(git_output.stderr.as_ref()));
        }
    } else {
        let git_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit_title)
            .arg("-m")
            .arg(message)
            .output()?;

        if git_output.status.success() {
            println!("{}", String::from_utf8_lossy(git_output.stdout.as_ref()));
        } else {
            eprintln!("{}", String::from_utf8_lossy(git_output.stderr.as_ref()));
        }
    }
    Ok(())
}
