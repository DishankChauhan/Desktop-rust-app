// src/csv_writer.rs
use std::fs::OpenOptions;
use csv::Writer;

pub fn save_to_csv(task_name: &str, mouse_positions: &[(i32, i32)]) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("tasks.csv")?;

    let mut wtr = Writer::from_writer(file);
    for pos in mouse_positions {
        wtr.write_record(&[task_name, &pos.0.to_string(), &pos.1.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}