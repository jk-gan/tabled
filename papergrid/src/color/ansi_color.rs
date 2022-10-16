use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

use super::Color;

/// The structure represents a ANSI color by suffix and prefix.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct AnsiColor<'a> {
    prefix: Cow<'a, str>,
    suffix: Cow<'a, str>,
}

impl<'a> AnsiColor<'a> {
    /// Constructs a new instance with suffix and prefix.
    ///
    /// They are not checked so you should make sure you provide correct ANSI.
    /// Otherwise you may want to use [`TryFrom`].
    ///
    /// [`TryFrom`]: std::convert::TryFrom
    // pub fn new(prefix: String, suffix: String) -> Self {
    //     Self { prefix, suffix }
    // }

    pub fn new_owned(prefix: String, suffix: String) -> Self {
        Self {
            prefix: Cow::Owned(prefix),
            suffix: Cow::Owned(suffix),
        }
    }

    pub fn new_borrows(prefix: &'a str, suffix: &'a str) -> Self {
        Self {
            prefix: Cow::Borrowed(prefix),
            suffix: Cow::Borrowed(suffix),
        }
    }

    /// Gets a reference to a prefix.
    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    /// Gets a reference to a suffix.
    pub fn get_suffix(&self) -> &str {
        &self.suffix
    }
}

impl<'a> std::convert::TryFrom<&str> for AnsiColor<'a> {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_ansi_color(value).ok_or(())
    }
}

impl<'a> std::convert::TryFrom<String> for AnsiColor<'a> {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl<'a> Color for AnsiColor<'a> {
    fn fmt_prefix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.prefix.fmt(f)
    }

    fn fmt_suffix(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.suffix.fmt(f)
    }
}

fn parse_ansi_color<'a>(s: &str) -> Option<AnsiColor<'a>> {
    let mut blocks = ansi_str::get_blocks(s);
    let block = blocks.next()?;

    let start = block.start().to_string();
    let end = block.end().to_string();

    Some(AnsiColor::new_owned(start, end))
}
