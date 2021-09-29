use std::fs::{File, OpenOptions};
use std::io::Write;
use serde::{Deserialize, Serialize};

pub fn write_to_file<T>(data: &T, path: &str) -> std::io::Result<()>
    where for<'a> T: Serialize + Deserialize<'a>
{
    let data = serde_json::to_string_pretty(data).unwrap();
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes()).expect("Unable to write data");
    Ok(())
}