use pipeviewer::{args::Args, read, stats, write, args};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut total_bytes = 0;

    loop {
        let buffer = match read::read(&args.infile) {
            Ok(x) if x.is_empty() => break,
            Ok(n) => n,
            _ => break,
        };
        stats::stats(args.silent, buffer.len(), &mut total_bytes, false);
        if !write::write(&args.outfile, &buffer)?{
            break;
        }
    }
    stats::stats(args.silent, 0, &mut total_bytes, true);
    Ok(())
}
