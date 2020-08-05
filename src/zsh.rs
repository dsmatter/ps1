use ansi_term::ANSIGenericString;
use std::fmt;

pub type ZshAnsiString<'a> = ZshGenericAnsiString<'a, str>;

/// ANSIGenericString wrapper which wraps ANSI style codes in '%{style%}' to help
/// zsh figure out correct line wrap points.
#[derive(Debug)]
pub struct ZshGenericAnsiString<'a, S>(pub ANSIGenericString<'a, S>)
where
    S: ToOwned + fmt::Display + 'a + ?Sized,
    S::Owned: fmt::Debug;

impl<'a, S> fmt::Display for ZshGenericAnsiString<'a, S>
where
    S: ToOwned + fmt::Display + 'a + ?Sized,
    S::Owned: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = self.0.style_ref();
        write!(
            f,
            "%{{{}%}}{}%{{{}%}}",
            style.prefix(),
            &*self.0,
            style.suffix()
        )
    }
}

impl<'a, S> From<ANSIGenericString<'a, S>> for ZshGenericAnsiString<'a, S>
where
    S: ToOwned + fmt::Display + 'a + ?Sized,
    S::Owned: fmt::Debug,
{
    fn from(s: ANSIGenericString<'a, S>) -> Self {
        ZshGenericAnsiString(s)
    }
}

pub trait IntoZsh<'a> {
    fn into_zsh(self) -> ZshAnsiString<'a>;
}

impl<'a> IntoZsh<'a> for ANSIGenericString<'a, str> {
    fn into_zsh(self) -> ZshAnsiString<'a> {
        ZshGenericAnsiString(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zsh_sample() {
        let string = ZshGenericAnsiString(ansi_term::Color::Green.bold().paint("foobar"));
        assert_eq!("%{\u{1b}[1;32m%}foobar%{\u{1b}[0m%}", format!("{}", string))
    }
}
