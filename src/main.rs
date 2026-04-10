#![doc = include_str!("../README.md")]

use argh::{self, FromArgs};
use crossterm::event::{self, KeyEventKind};
use crossterm::terminal;
use std::io;

/// Simple crossplatform program that is waiting until a key is pressed
#[derive(FromArgs)]
struct Args {
    /// expected keys to press to exit; if skipped, any key is expected
    #[argh(option, short = 'e')]
    expect: Option<String>,

    /// a help message for user
    #[argh(option, short = 'p')]
    prompt: Option<String>,

    /// suppress any prompt printing, even a new line
    #[argh(switch, short = 's')]
    silent: bool,

    /// print info about key presses
    #[argh(switch, short = 'v')]
    verbose: bool,
}

fn main() -> io::Result<()> {
    let args: Args = argh::from_env();
    if !args.silent {
        if let Some(prompt) = args.prompt {
            println!("{prompt}");
        } else {
            println!("Press any key...");
        }
    }

    terminal::enable_raw_mode()?;
    loop {
        let ev = event::read()?;
        if let Some(key_event) = ev.as_key_event()
            && key_event.kind == KeyEventKind::Press
        {
            if args.verbose {
                eprintln!("{key_event:?}");
            }
            match (&args.expect, key_event.code.as_char()) {
                (Some(expect), Some(input)) if expect.contains(input) => break,
                (None, ..) => break,
                _ => continue,
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}
