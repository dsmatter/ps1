use ansi_term::{ANSIString, Color};
use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tico::tico;

use ps1::duration::format_duration;
use ps1::ffi::get_user_id;
use ps1::git;
use ps1::zsh::{IntoZsh, ZshGenericAnsiString};

macro_rules! format_opts {
    ($($e:expr),+) => {{
        let mut result = String::new();
        $(
            if let Some(s) = $e {
                result.push(' ');
                result.push_str(&s.to_string());
            }
        )*
        result
    }};
}

fn main() {
    let mut args = std::env::args();
    let last_exit_code = {
        let arg = args.nth(1).expect("last exit code argument");
        if arg == "dump_time" {
            dump_time();
            return;
        }
        arg
    };

    let hostname = args.next().expect("hostname argument");
    let command_duration_ms = args.next().and_then(|s| s.parse::<u64>().ok());

    let user_id = get_user_id();
    let cwd = std::env::current_dir().unwrap();
    let home_dir = dirs::home_dir();
    let home_dir = home_dir.as_ref().and_then(|d| d.to_str());
    let cwd_short = tico(cwd.to_str().unwrap(), home_dir);
    let git_status = git::status(&cwd).map(GitStatus);
    let command_duration = command_duration_ms.map(|ms| {
        Color::Yellow
            .bold()
            .paint(format_duration(Duration::from_millis(ms)))
            .into_zsh()
    });

    let prompt_char = ZshGenericAnsiString(if user_id == 0 {
        Color::Red.bold().paint("#")
    } else {
        Color::Red.paint("⭑")
    });

    if &last_exit_code != "0" {
        print!(
            "💥 {}",
            Color::Red
                .bold()
                .paint(format!("[{}]", last_exit_code))
                .into_zsh()
        );
    }

    print!(
        "\n{} {} {}{}{}\n{} {}\u{00A0}\u{00A0}",
        Color::White.paint("⎡").into_zsh(),
        Color::Green.bold().paint(hostname).into_zsh(),
        Color::Green.paint(cwd_short).into_zsh(),
        Color::White.paint("⎤").into_zsh(),
        format_opts!(git_status, command_duration),
        Color::White.paint("⎣").into_zsh(),
        prompt_char,
    );
}

struct GitStatus(git::Status);

impl fmt::Display for GitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut write = |s: ANSIString| ZshGenericAnsiString(s).fmt(f);
        let status = &self.0;

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

        if let Some(upstream) = status.upstream.as_ref() {
            if upstream.commits_ahead > 0 {
                write(Color::Red.paint(format!(" ▲{}", upstream.commits_ahead)))?;
            }
            if upstream.commits_behind > 0 {
                write(Color::Red.paint(format!(" ▼{}", upstream.commits_behind)))?;
            }
        }

        if status.files.untracked_files > 0 {
            write(Color::Yellow.paint(format!(" ❖{}", status.files.untracked_files)))?;
        }

        Ok(())
    }
}

fn dump_time() {
    println!(
        "{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("epoch")
            .as_millis()
    );
}
