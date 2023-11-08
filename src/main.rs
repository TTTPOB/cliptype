#![windows_subsystem = "windows"]

use anyhow::{anyhow, Result};
use arboard::Clipboard;
use enigo::*;
use std::{thread, time::Duration};

fn main() -> Result<()> {
    let mut clipboard = Clipboard::new()?;
    let text = clipboard
        .get_text()
        .map_err(|_| anyhow!("Clipboard does not contain text data"))?;

    let mut enigo = Enigo::new();
    thread::sleep(Duration::from_secs(1)); // Wait before typing
    enigo.key_sequence(&text);

    Ok(())
}
