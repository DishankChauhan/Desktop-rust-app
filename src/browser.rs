// src/browser.rs
use webbrowser;

pub fn open_browser(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    webbrowser::open(url)?;
    Ok(())
}