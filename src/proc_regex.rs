//--- RegExp version -------------------------------------------------------------------

extern crate itertools;
extern crate regex;

use std;
use std::io::{BufRead, Write};

use self::regex::*;

pub fn create_regex() -> Regex {
    let hex_byte_re = r"([[:xdigit:]]{2})";
    Regex::new(&("#".to_string() + &hex_byte_re.repeat(3))).unwrap()
}

fn scale_hex_byte_str(hex_byte_str: &str, scale: f64) -> String {
    let byte = u32::from_str_radix(hex_byte_str, 16).unwrap();
    let n = (scale * f64::from(byte)).round() as u8;
    format!("{:02x}", std::cmp::min(n, 255))
}

fn process_line(line: &str, re: &Regex, scale: f64) -> String {
    let scaled = re.replace_all(&line, |caps: &Captures| {
        "#".to_string()
            + &itertools::join(
                caps.iter()
                    .skip(1)
                    .map(|s| scale_hex_byte_str(s.unwrap().as_str(), scale)),
                "",
            )
    });
    scaled.into_owned()
}

pub fn process(reader: impl BufRead, writer: impl Write, scale: f64) -> Result<(), &'static str> {
    let re = create_regex();
    process_regex(reader, writer, scale, &re)
}

pub fn process_regex(
    reader: impl BufRead,
    mut writer: impl Write,
    scale: f64,
    re: &Regex,
) -> Result<(), &'static str> {
    for line in reader.lines() {
        let scaled = process_line(&line.unwrap(), &re, scale);
        writer.write_all(scaled.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
    }
    Ok(())
}
