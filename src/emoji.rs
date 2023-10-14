use std::fmt;

/// List of emojis
pub const EMOJIS: [Emoji<'static>; 13] = [
    Emoji::new("📦️", ":build:", "Build and packaging related changes"),
    Emoji::new("👷", ":ci:", "Update to the continuous integration system"),
    Emoji::new("🔧", ":config:", "Changes to various configuration files"),
    Emoji::new("📝", ":docs:", "Documentation related changes"),
    Emoji::new("✨", ":feat:", "A new feature"),
    Emoji::new("🐛", ":fix:", "A bug fix"),
    Emoji::new("⚡️", ":perf:", "Performance improvements"),
    Emoji::new("♻️", ":refactor:", "Code changes without fixes or features"),
    Emoji::new("🎉", ":release:", "Release a new version"),
    Emoji::new("🔒️", ":security:", "Fix security or privacy issues"),
    Emoji::new("💄", ":style:", "Better styling"),
    Emoji::new("🚨", ":test:", "Add or update tests"),
    Emoji::new("🚧", ":wip:", "Work-in-progress code changes"),
];

/// Struct of emoji data
#[derive(Debug, Clone)]
pub struct Emoji<'a> {
    pub emoji: &'a str,
    pub code: &'a str,
    pub description: &'a str,
}

impl<'a> Emoji<'a> {
    /// Create a new emoji
    pub const fn new(emoji: &'a str, code: &'a str, description: &'a str) -> Self {
        Self {
            emoji,
            code,
            description,
        }
    }
}

impl<'a> fmt::Display for Emoji<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.emoji, self.code, self.description)
    }
}
