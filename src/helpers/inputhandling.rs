use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_string(day: u8) -> Result<String, Box<dyn Error>> {
    let pathstr = format!(".\\src\\day{}\\input.txt", day);
    let path = Path::new(&pathstr);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)  
}

#[allow(dead_code)]
pub fn parse_input<T>(
    day: u8,
    parser: fn(&str) -> Result<T, Box<dyn Error>>,
) -> Result<Vec<T>, Box<dyn Error>> {
    get_string(day)?
        .lines()
        .map(|s| s.trim())
        .map(|s| parser(s))
        .collect()
}

#[allow(dead_code)]
pub fn get_input_chars(
    day: u8,
) -> Result<Vec<char>, Box<dyn Error>> {
    let contents = get_string(day)?;
      Ok(contents.trim()
      .chars()
      .collect::<Vec<char>>())
}

#[allow(dead_code)]
pub fn parse_input_per_line<T>(
    day: u8,
    parser: fn(&str) -> Result<T, Box<dyn Error>>,
) -> Result<Vec<T>, Box<dyn Error>> {
    get_string(day)?
        .lines()
        .map(|s| parser(s.trim()))
        .collect()
}

#[allow(dead_code)]
pub fn parse_input_csv_per_line<T>(
    day: u8,
    parser: fn(&str) -> Result<T, Box<dyn Error>>,
) -> Result<Vec<Vec<T>>, Box<dyn Error>> {
    get_string(day)?
        .lines()
        .map(|s| s.trim().split(",").map(|x| parser(x)).collect())
        .collect()
}

#[allow(dead_code)]
pub fn parse_csv_input<T>(
    day: u8,
    parser: fn(&str) -> Result<T, Box<dyn Error>>,
) -> Result<Vec<T>, Box<dyn Error>> {
    get_string(day)?
        .split(",")
        .map(|s| s.trim())
        .map(|s| parser(s))
        .collect()
}
