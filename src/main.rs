use ps1::ffi::get_user_id;
use ps1::git;
use ps1::zsh::{ZshAnsiString, ZshGenericAnsiString};

use ansi_term::{ANSIString, Color};
use dirs;
use std::fmt;
use tico::tico;

fn main() {
    let mut args = std::env::args();
    let last_exit_code = args.nth(1).expect("last exit code argument");
    let hostname = args.next().expect("hostname argument");

    let user_id = get_user_id();
    let cwd = std::env::current_dir().unwrap();
    let home_dir = dirs::home_dir();
    let home_dir = home_dir.as_ref().and_then(|d| d.to_str());
    let cwd_short = tico(cwd.to_str().unwrap(), home_dir);
    let git_status = GitStatus(git::status(&cwd));
    let prompt_char = ZshGenericAnsiString(if user_id == 0 {
        Color::Red.bold().paint("#")
    } else {
        Color::Red.paint("⭑")
    });

    if &last_exit_code != "0" {
        print!(
            "💩 💩 💩  {}",
            ZshGenericAnsiString(Color::Red.bold().paint(format!("[{}]", last_exit_code)))
        );
    }

    print!(
        "\n{} {} {}{} {}\n{} {}\u{00A0}\u{00A0}",
        top_left_bracket(),
        ZshGenericAnsiString(Color::Green.bold().paint(hostname)),
        ZshGenericAnsiString(Color::Green.paint(cwd_short)),
        top_right_bracket(),
        git_status,
        bottom_left_bracket(),
        prompt_char,
    );
}

struct GitStatus(Option<git::Status>);

impl fmt::Display for GitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut write = |s: ANSIString| ZshGenericAnsiString(s).fmt(f);

        let status = match &self.0 {
            Some(status) => status,
            _ => return fmt::Result::Ok(()),
        };

        match &status.ref_name {
            git::RefName::Branch(name) => write(Color::Blue.bold().paint(name)),
            git::RefName::Hash(hash) => write(Color::Yellow.bold().paint(hash)),
        }?;

        if status.files.uncommitted_files == 0 {
            write(Color::Green.bold().paint(" ✓"))
        } else {
            write(
                Color::Red
                    .bold()
                    .paint(format!(" ±{}", status.files.uncommitted_files)),
            )
        }?;

        match status.upstream.as_ref().map(|u| u.commits_ahead) {
            Some(n) if n > 0 => {
                write(Color::Red.paint(format!(" ▲{}", n)))?;
            }
            _ => (),
        };

        match status.upstream.as_ref().map(|u| u.commits_behind) {
            Some(n) if n > 0 => {
                write(Color::Red.paint(format!(" ▼{}", n)))?;
            }
            _ => (),
        };

        if status.files.untracked_files == 0 {
            Ok(())
        } else {
            write(Color::Yellow.paint(format!(" ❖{}", status.files.untracked_files)))
        }
    }
}

pub fn top_left_bracket() -> ZshAnsiString<'static> {
    Color::White.paint("⎡").into()
}

pub fn top_right_bracket() -> ZshAnsiString<'static> {
    Color::White.paint("⎤").into()
}

pub fn bottom_left_bracket() -> ZshAnsiString<'static> {
    Color::White.paint("⎣").into()
}
