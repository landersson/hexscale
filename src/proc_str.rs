//--- rust 'String' version -------------------------------------------------------------------
//
use std::io::{BufRead, Write};
use std::str;

fn is_hex_color(slice: &str) -> bool {
    if slice.len() < 7 || slice.as_bytes()[0] != b'#' {
        return false;
    }
    for c in slice.chars().skip(1) {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}

fn str_chunks<'a>(s: &'a str, n: usize) -> Box<Iterator<Item = &'a str> + 'a> {
    Box::new(s.as_bytes().chunks(n).map(|c| str::from_utf8(c).unwrap()))
}

fn scale_hex_byte(hex: &str, scale: f64) -> String {
    let byte = i32::from_str_radix(hex, 16).unwrap();
    //let n = (scale * byte as f64).round() as i32;
    let n = (scale * f64::from(byte)).round() as i32;
    format!("{:02x}", std::cmp::min(n, 255))
}

fn scale_hex_slice(hex: &str, scale: f64) -> String {
    let mut s: String = "#".to_string();

    for h in str_chunks(&hex[1..], 2) {
        s.push_str(&scale_hex_byte(h, scale));
    }
    s
}

fn scale_line(line: &str, scale: f64) -> String {
    let mut s2 = "".to_string();
    let mut skip = 0;
    for (i, c) in line.chars().enumerate() {
        let slice = &line[i..(std::cmp::min(i + 7, line.len()))];

        if is_hex_color(slice) {
            s2.push_str(&scale_hex_slice(slice, scale));
            skip = 6;
        } else if skip == 0 {
            s2.push(c);
        } else {
            skip -= 1;
        }
    }
    s2
}

pub fn process(
    reader: impl BufRead,
    mut writer: impl Write,
    scale: f64,
) -> Result<(), &'static str> {
    for line in reader.lines() {
        let line = scale_line(&line.unwrap(), scale);
        writer.write_all(line.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
    }
    Ok(())
}
