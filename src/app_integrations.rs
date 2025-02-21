use std::process::Command;

pub fn open_figma() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("open")
        .arg("-a")
        .arg("Figma")
        .spawn()?;
    Ok(())
}

pub fn open_zoom() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("open")
        .arg("-a")
        .arg("zoom.us") // Use "zoom.us" for macOS
        .spawn()?;
    Ok(())
}