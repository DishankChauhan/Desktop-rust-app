use std::fs;
use std::process::Command;

pub fn start_recording(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create the output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    let output_file = format!("{}/output.mp4", output_dir);

    // Check if the file already exists and delete it
    if fs::metadata(&output_file).is_ok() {
        fs::remove_file(&output_file)?;
    }

    // Start screen recording
    Command::new("screencapture")
        .arg("-v")
        .arg(&output_file)
        .spawn()?;
    Ok(())
}

pub fn stop_recording() -> Result<(), Box<dyn std::error::Error>> {
    // Send SIGINT to screencapture (equivalent to Ctrl+C)
    Command::new("pkill")
        .arg("-SIGINT")
        .arg("screencapture")
        .spawn()?;
    Ok(())
}
