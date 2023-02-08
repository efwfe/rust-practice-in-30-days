// static B: [u8; 10] = [99, 97, 114,121,111,111,22,54,23,2];
// static C: [u8; 10] = [12,121,44,2,46,77,33,131,34,66];
// fn main() {
//     let a =42;
//     let b = &B;
//     let c = &C;

//     println!("a:{}, b:{:p}, c:{:p},", a,b,c); // ：P 格式化输出指针
// }

// 

use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114,121,111,111,22,54,23,2];
static C: [u8; 10] = [12,121,44,2,46,77,33,131,34,66];

fn main(){
    let a: usize = 42;// uszie的实际宽度为内存地址的宽度，内存地址宽度取决于cpu
    let b: &[u8;10] = &B;
    let c: Box<[u8]> = Box::new(C); // Box<[u8]>是一个装箱的字节切片，当某个值装入其中所有权就转移给了box的所有者
    println!("a (an unsigned integer");
    println!("  location: {:p}", &a);
    println!("  size:  {:?} bytes", size_of::<usize>());
    println!("  value: {:?}", a);
    println!();

    println!("b (a reference to B)");
    println!("  location: {:p}", &b);
    println!("  size:     {:?} bytes", size_of::<&[u8;10]>());
    println!("  points to: {:p}", b);
    println!();

    println!("c (a 'box' for C)");
    println!("  location: {:p}", &c);
    println!("  size: {:?} bytes", size_of::<Box<[u8]>>());
    println!("  points to:{:p}", c);
    println!();

    println!("B (an array of 10 bytes)");
    println!("  location: {:p}",&B);
    println!("  size: {:?} bytes", size_of::<[u8;10]>());
    println!("  value: {:?}", B);
    println!();

    println!("C (an array of 11 bytes)");
    println!("  location: {:p}", &C);
    println!("  size: {:?} bytes", size_of::<[u8;10]>());
    println!("  value: {:?}", C);


}