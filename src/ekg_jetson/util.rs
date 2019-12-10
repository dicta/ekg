use std::{fs, io, str};
use std::path::Path;


pub fn string_from_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let p_ref = path.as_ref();
    let value = fs::read_to_string(&path)
        .or_else(|e| {
            Err(io::Error::new(e.kind(),
                format!("Could not read file: {}", p_ref.display())))
            })?
        .trim_end_matches("\n")
        .to_string();
    Ok(value)
}

pub fn value_from_file<T: str::FromStr, P: AsRef<Path>>(path: P) -> io::Result<T> {
    let p_ref = path.as_ref();
    fs::read_to_string(&path)
        .or_else(|e| {
            Err(io::Error::new(e.kind(),
                format!("Could not read file: {}", p_ref.display())))
            })?
        .trim_end_matches("\n")
        .parse()
        .and_then(|n| Ok(n))
        .or_else(|_| {
            Err(io::Error::new(io::ErrorKind::Other,
                               format!("File: Could not parse numeric value from {}", p_ref.display())))
        })
}
