//--- low level, u8 version ------------------------------------------------------------

use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::str;

fn make_hex_digit(x: u8) -> u8 {
    if x < 0xa {
        return b'0' + x;
    }
    assert!(x < 16);
    b'a' + (x - 10)
}

fn scale_hex_byte(hb: &mut [u8], scale: f64) {
    let byte = i32::from_str_radix(str::from_utf8(hb).unwrap(), 16).unwrap();
    let n = (scale * f64::from(byte)).round() as i32;
    let n = std::cmp::min(n, 255) as u8;
    hb[0] = make_hex_digit(n >> 4);
    hb[1] = make_hex_digit(n & 0xF);
}

fn scale_rgb(buf: &mut [u8], scale: f64) {
    if buf.len() < 6 {
        return;
    }
    for ch in buf.iter().take(6) {
        if !ch.is_ascii_hexdigit() {
            return;
        }
    }
    for i in 0..3 {
        scale_hex_byte(&mut buf[2 * i..2 * i + 2], scale);
    }
}

pub fn process<T: BufRead, U: Write>(
    reader: &mut T,
    writer: &mut U,
    scale: f64,
) -> Result<(), &'static str> {
    let chunk_size: u64 = 8 * 1024;

    let mut buf: Vec<u8> = Vec::with_capacity(chunk_size as usize);

    loop {
        match reader.take(chunk_size).read_until(b'#', &mut buf) {
            Ok(bytes_read) => {
                //println!("Read {} bytes: bufsz={}/{}", bytes_read, buf.len(), buf.capacity());
                if bytes_read == 0usize {
                    break;
                }
                if *buf.last().unwrap() == b'#' {
                    let i1 = buf.len();

                    reader.take(6).read_until(0, &mut buf).unwrap();

                    //println!("BUFFER: {:?}, {}", buf, xx);

                    let i2 = buf.len();
                    scale_rgb(&mut buf[i1..i2], scale);
                }
                writer.write_all(&buf).unwrap();
                //println!("Read {} bytes: bufsz={}/{}", bytes_read, buf.len(), buf.capacity());
                buf.clear();
            }
            Err(error) => panic!("Error reading data from input file: {:?}", error),
        }
    }
    Ok(())
}
