use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::Indent;

#[derive(Deserialize, Serialize, SmartDefault, Debug, Clone)]
/// Configuration for rsnfmt
#[serde(default)]
pub struct Config {
    /// Max line width
    #[default = 60]
    pub max_width: usize,
    /// Max level of inline nesting
    #[default = 2]
    pub max_inline_level: usize,
    /// Normalize all comments to a specific format
    pub normalize_comments: NormalizeComments,
    /// Wrap comments longer than `max_width`
    pub wrap_comments: bool,
    /// Should formatting preserve empty lines
    pub preserve_empty_lines: PreserveEmptyLines,
    /// Inherit parent/global configuration
    #[default = true]
    pub inherit: bool,
    /// Line ending
    pub line_ending: LineEnding,
    /// Indentation width
    #[default = 4]
    pub indent: usize,
    /// Use `\t` to indent
    pub hard_tab: bool,
}

impl Config {
    pub(crate) fn indent(&self) -> Indent {
        Indent {
            level: 0,
            hard_tab: self.hard_tab,
            width: self.indent,
        }
    }

    pub(crate) fn line_ending(&self, source: &str) -> &'static str {
        #[cfg(not(windows))]
        const PLATFORM: &str = "\n";
        #[cfg(windows)]
        const PLATFORM: &str = "\r\n";
        match self.line_ending {
            LineEnding::Detect => {
                if let Some(idx) = source.find('\n') {
                    if matches!(source.as_bytes().get(idx - 1), Some(b'\r')) {
                        "\r\n"
                    } else {
                        "\n"
                    }
                } else {
                    PLATFORM
                }
            }
            LineEnding::Platform => PLATFORM,
            LineEnding::Lf => "\n",
            LineEnding::CrLf => "\r\n",
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy)]
/// Should comments be normalized
pub enum NormalizeComments {
    /// Make all comments block comments (`/* */`)
    Block,
    /// Make all comments line comments (`//`)
    Line,
    /// Do not normalize Comments
    #[default]
    No,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy)]
/// Should empty lines be preserved
pub enum PreserveEmptyLines {
    /// Reduce multiple empty lines to a single one
    One,
    /// Preserve all empty lines
    #[default]
    All,
    /// Do not preserve any empty lines
    None,
}

impl PreserveEmptyLines {
    pub(crate) fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy)]
/// Line endings to use
pub enum LineEnding {
    /// Detect line endings from input
    ///
    /// Uses the first line ending encountered, if the file does not have any
    /// line breaks, falls back to [`Platform`](Self::Platform).
    #[default]
    Detect,
    /// Use platform line endings
    ///
    /// - `\n\r` on Windows
    /// - `\n` everywhere else
    Platform,
    /// Use unix line endings (`\n`)
    Lf,
    /// Use windows line endings (`\n\r`)
    CrLf,
}
