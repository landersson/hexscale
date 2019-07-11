#[macro_use]
extern crate bencher;
extern crate hexscale;

use bencher::Bencher;
use hexscale::proc_regex;
use hexscale::proc_str;
use hexscale::proc_u8;
use std::io::{Seek, SeekFrom};

static BENCH_DATA: &'static str = include_str!("../data/bench.txt");

fn perf_regex(b: &mut Bencher) {
    let re = proc_regex::create_regex();
    let scale: f64 = bencher::black_box(0.5);

    let mut cur_in = std::io::Cursor::new(BENCH_DATA);
    let mut cur_out = std::io::Cursor::new(Vec::with_capacity(BENCH_DATA.len()));

    b.iter(|| {
        proc_regex::process_regex(&mut cur_in, &mut cur_out, scale, &re).unwrap();
        cur_in.seek(SeekFrom::Start(0)).unwrap();
        cur_out.seek(SeekFrom::Start(0)).unwrap();
    })
}

fn perf_str(b: &mut Bencher) {
    let scale: f64 = bencher::black_box(0.5);

    let mut cur_in = std::io::Cursor::new(BENCH_DATA);
    let mut cur_out = std::io::Cursor::new(Vec::with_capacity(BENCH_DATA.len()));

    b.iter(|| {
        proc_str::process(&mut cur_in, &mut cur_out, scale).unwrap();
        cur_in.seek(SeekFrom::Start(0)).unwrap();
        cur_out.seek(SeekFrom::Start(0)).unwrap();
    })
}

fn perf_u8(b: &mut Bencher) {
    let scale: f64 = bencher::black_box(0.5);
    let mut cur_in = std::io::Cursor::new(BENCH_DATA);
    let mut cur_out = std::io::Cursor::new(Vec::with_capacity(BENCH_DATA.len()));

    b.iter(|| {
        proc_u8::process(&mut cur_in, &mut cur_out, scale).unwrap();
        cur_in.seek(SeekFrom::Start(0)).unwrap();
        cur_out.seek(SeekFrom::Start(0)).unwrap();
    })
}
benchmark_group!(benches, perf_regex, perf_str, perf_u8);
benchmark_main!(benches);
