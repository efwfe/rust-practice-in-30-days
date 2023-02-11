fn parity_bit(bytes: &[u8])->u8{
    let mut n_ones : u32 = 0;
    for byte in bytes{
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (ob:{:08b} has {} one bits", byte, byte, ones);
    }
    (n_ones % 2 == 0) as u8
}

fn main() {
    let abc = b"abc";
    println!("input: {:?}", abc);
    println!("output: {:08x}", parity_bit(abc));
    println!();

    let abcd = b"abf";
    println!("input: {:?}", abcd);
    println!("result:{:08b}", parity_bit(abcd));
}

