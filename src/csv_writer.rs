use std::fs::OpenOptions;
use csv::Writer;

pub fn save_to_csv(
    task_name: &str,
    start_time: &str,
    end_time: &str,
    mouse_positions: &[(i32, i32)],
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = format!("{}/tasks.csv", output_dir);
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    let mut wtr = Writer::from_writer(file);
    for pos in mouse_positions {
        wtr.write_record(&[
            task_name,
            start_time,
            end_time,
            &pos.0.to_string(),
            &pos.1.to_string(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}