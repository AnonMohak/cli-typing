#[allow(dead_code, unused_imports)] // The UI uses the engine from M2.
mod engine;

use std::io::{self, Write};

use clap::Parser;
use ratatui::crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal,
};

/// A monkeytype-style typing test for the terminal.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Print each key event, to check how the terminal sends keys.
    #[arg(long)]
    keys: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    if cli.keys {
        return key_test();
    }
    println!("typer: the typing test is not ready yet. Use --keys to test key input.");
    Ok(())
}

/// Print every key event in raw mode. Ctrl+C quits.
fn key_test() -> io::Result<()> {
    println!("Press keys to see their events. Press Ctrl+C to quit.");
    terminal::enable_raw_mode()?;
    let result = (|| -> io::Result<()> {
        let mut out = io::stdout();
        loop {
            if let Event::Key(key) = event::read()? {
                write!(
                    out,
                    "{:<8} code={:?} modifiers={:?}\r\n",
                    format!("{:?}", key.kind),
                    key.code,
                    key.modifiers
                )?;
                out.flush()?;
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    return Ok(());
                }
            }
        }
    })();
    terminal::disable_raw_mode()?;
    result
}
