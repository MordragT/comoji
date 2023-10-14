//! Handles everything that concerns the configuration.
use crate::error::*;
use clap::{Parser, ValueEnum};
use dialoguer::{theme::Theme, Confirm, Select};
use serde::{Deserialize, Serialize};

/// Emojiformat which should be used in the commit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ValueEnum)]
pub enum EmojiFormat {
    /// CODE would like :smile:
    Code,
    /// EMOJI would be the unicode character itself
    Emoji,
}

impl Default for EmojiFormat {
    fn default() -> Self {
        EmojiFormat::Code
    }
}

/// Stores the configuration for the cli
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Config {
    #[clap(short, long)]
    pub auto_add: bool,
    #[clap(value_enum, default_value_t = EmojiFormat::Code)]
    pub emoji_format: EmojiFormat,
    #[clap(short = 'p', long)]
    pub scope_prompt: bool,
    #[clap(short, long)]
    pub signed_commit: bool,
    #[clap(short, long)]
    pub referring_issue: bool,
}

impl Config {
    /// Starts the prompts for configuring and uses the current values as defaults.
    pub fn prompt(theme: &dyn Theme) -> ComojiResult<Self> {
        let auto_add = Confirm::with_theme(theme)
            .with_prompt("Enable automatic \"git add .\":")
            .default(false)
            .interact()?;

        let selection = Select::with_theme(theme)
            .with_prompt("Select how emojis should be used in commits:")
            .items(&["release:", "🎉"])
            .default(0)
            .interact()?;

        let emoji_format = match selection {
            0 => EmojiFormat::Code,
            _ => EmojiFormat::Emoji,
        };

        let scope_prompt = Confirm::with_theme(theme)
            .with_prompt("Enable scope prompt:")
            .default(false)
            .interact()?;

        let signed_commit = Confirm::with_theme(theme)
            .with_prompt("Enable signed commits:")
            .default(false)
            .interact()?;

        let reffering_issue = Confirm::with_theme(theme)
            .with_prompt("Enable referring issue prompt:")
            .default(false)
            .interact()?;

        Ok(Self {
            auto_add,
            emoji_format,
            scope_prompt,
            signed_commit,
            referring_issue: reffering_issue,
        })
    }
}
