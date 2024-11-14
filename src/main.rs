extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: <source> <target>");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    // Copy input to encoder
    copy(&mut input, &mut encoder).unwrap();
    encoder.finish().unwrap();

    // Output file metadata
    println!("Source len: {}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {}", File::open(args().nth(2).unwrap()).unwrap().metadata().unwrap().len());
    
    // Elapsed time in seconds and nanoseconds
    let duration = start.elapsed();
    println!("Elapsed: {} seconds and {} nanoseconds", duration.as_secs(), duration.subsec_nanos());
}
