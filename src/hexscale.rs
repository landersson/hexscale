//#![feature(test)]
#![allow(dead_code)]

extern crate hexscale;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::str;

use hexscale::proc_regex;

fn run(args: Vec<String>) -> Result<(), &'static str> {
    let reader = open_input_file(&args[1])?;
    let scale: f64 = args[2].to_string().parse().unwrap();
    let writer = open_output_file(&args[3])?;
    proc_regex::process(reader, writer, scale)
}

fn open_input_file(name: &str) -> Result<BufReader<File>, &'static str> {
    let input = File::open(name).map_err(|_| "input file missing")?;
    Ok(BufReader::new(input))
}

fn open_output_file(name: &str) -> Result<BufWriter<File>, &'static str> {
    let output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(name)
        .unwrap();
    Ok(BufWriter::new(output))
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() >= 4 {
        if let Err(e) = run(args) {
            println!("{}", e)
        }
    } else {
        println!("Usage: {} [IN_FILE] [SCALE] [OUT_FILE]", &args[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hexscale::proc_regex;
    use hexscale::proc_str;
    use hexscale::proc_u8;

    static TEST_IN: &'static str = "L1: C=#8020a0,#a0b0c0\nL2: C=#303030\nL3: NO #C0L0R\n";
    static TEST_OUT: &'static str = "L1: C=#401050,#505860\nL2: C=#181818\nL3: NO #C0L0R\n";

    #[test]
    fn process_regex() {
        let mut cur_out = std::io::Cursor::new(Vec::with_capacity(TEST_IN.len()));
        proc_regex::process(std::io::Cursor::new(TEST_IN), &mut cur_out, 0.5).unwrap();
        assert_eq!(
            str::from_utf8(cur_out.get_ref()).unwrap(),
            TEST_OUT.to_string()
        )
    }

    #[test]
    fn test_process_str() {
        let mut cur_out = std::io::Cursor::new(Vec::with_capacity(TEST_IN.len()));
        proc_str::process(&mut std::io::Cursor::new(TEST_IN), &mut cur_out, 0.5).unwrap();
        assert_eq!(
            str::from_utf8(cur_out.get_ref()).unwrap(),
            TEST_OUT.to_string()
        )
    }

    #[test]
    fn test_process_u8() {
        let mut cur_out = std::io::Cursor::new(Vec::with_capacity(TEST_IN.len()));
        proc_u8::process(&mut std::io::Cursor::new(TEST_IN), &mut cur_out, 0.5).unwrap();
        assert_eq!(
            str::from_utf8(cur_out.get_ref()).unwrap(),
            TEST_OUT.to_string()
        )
    }

}
