use std::io;
use std::io::{Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main(){
    let mut total_bytes = 0;
    loop{
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer){
            Ok(0) => break,
            Ok(n)=>n,
             _=>break,
        };
        total_bytes += num_read;
        io::stdout().write_all(&buffer).unwrap();
    }
    eprintln!("total: {}", total_bytes);

}