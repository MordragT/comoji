//! Collection of all used prompts
use std::process::Command;

use crate::config::Config;
use crate::emoji::EMOJIS;
use crate::error::ComojiResult;
use crate::{config::EmojiFormat, emoji::Emoji};
use dialoguer::theme::Theme;
use dialoguer::{FuzzySelect, Input};
use miette::{IntoDiagnostic, Result};

pub struct Commit<'a> {
    emoji: Emoji<'a>,
    title: String,
    message: String,
    scope: Option<String>,
    issue: Option<String>,
    config: &'a Config,
}

impl<'a> Commit<'a> {
    pub fn prompt(theme: &dyn Theme, config: &'a Config) -> ComojiResult<Self> {
        let selection = FuzzySelect::with_theme(theme)
            .with_prompt("Choose a comoji:")
            .items(&EMOJIS)
            .interact()?;

        let emoji = EMOJIS
            .into_iter()
            .enumerate()
            .find_map(|(i, emoji)| if i == selection { Some(emoji) } else { None })
            .unwrap();

        let scope = if config.scope_prompt {
            Some(
                Input::with_theme(theme)
                    .with_prompt("Enter the scope of current changes:")
                    .allow_empty(true)
                    .validate_with(|v: &String| {
                        if v.contains('`') {
                            Err("Enter a valid scope")
                        } else {
                            Ok(())
                        }
                    })
                    .interact()?,
            )
        } else {
            None
        };

        let title = Input::with_theme(theme)
            .with_prompt("Enter the commit title:")
            .validate_with(|v: &String| {
                if v.contains('`') || v.is_empty() {
                    Err("Enter a valid title")
                } else {
                    Ok(())
                }
            })
            .interact()?;

        let message = Input::with_theme(theme)
            .with_prompt("Enter the commit message:")
            .validate_with(|v: &String| {
                if v.contains('`') {
                    Err("Enter a valid message")
                } else {
                    Ok(())
                }
            })
            .allow_empty(true)
            .interact()?;

        let issue = if config.referring_issue {
            Some(
                Input::with_theme(theme)
                    .with_prompt("Enter the referring issue:")
                    .validate_with(|v: &String| {
                        if v.contains('`') {
                            Err("Enter a valid issue")
                        } else {
                            Ok(())
                        }
                    })
                    .allow_empty(true)
                    .interact()?,
            )
        } else {
            None
        };

        Ok(Self {
            emoji,
            title,
            message,
            scope,
            issue,
            config,
        })
    }

    pub fn build(self) -> Result<()> {
        let Self {
            emoji,
            title,
            message,
            scope,
            issue,
            config,
        } = self;

        let mut commit_title = String::new();
        if config.emoji_format == EmojiFormat::Code {
            commit_title += emoji.code;
        } else {
            commit_title += emoji.emoji;
        }
        commit_title += " ";
        if let Some(scope) = scope {
            if !scope.is_empty() {
                commit_title += scope.as_str();
                commit_title += ": ";
            }
        }
        commit_title += title.as_str();

        if let Some(issue) = issue {
            if !issue.is_empty() {
                commit_title += " (";
                commit_title += issue.as_str();
                commit_title += ")";
            }
        }

        if config.auto_add {
            Command::new("git")
                .arg("add")
                .arg(".")
                .output()
                .into_diagnostic()?;
        }

        if config.signed_commit {
            let git_output = Command::new("git")
                .arg("commit")
                .arg("-S")
                .arg("-m")
                .arg(commit_title)
                .arg("-m")
                .arg(message)
                .output()
                .into_diagnostic()?;

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
                .output()
                .into_diagnostic()?;

            if git_output.status.success() {
                println!("{}", String::from_utf8_lossy(git_output.stdout.as_ref()));
            } else {
                eprintln!("{}", String::from_utf8_lossy(git_output.stderr.as_ref()));
            }
        }

        Ok(())
    }
}
