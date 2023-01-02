/* 
 * Rust每个值有确切的数据类型，总体可以分为两类：基本类型和复合类型。
 * 基本类型
 *  数值类型：有符号整数；无符号整数；浮点数；以及有理数，复数；
 *  字符串：字符串字面量和字符串切片 &str
 *  布尔类型：true和false
 *  字符类型：单个unicode字符
 *  单元类型：（）
 * 
 *  Rust是一门静态类型语言，编译器开可以根据变量的值和上下文中的使用方式来推到出变量的类型
 *  但有时候需要手动去给予一个类型标注。
 * */

use day2_basic_types::number_type;

fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz:(f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc (f32)");
    println!("  0.1+0.2:{:x}", (abc.0 + abc.1).to_bits());
    println!("      0.3:{:x}", abc.2.to_bits());
    println!("xyz (f64");
    println!("  0.1+0.2:{:x}", (xyz.0 + xyz.1).to_bits());
    println!("      0.3:{:x}", xyz.2.to_bits());

    // NaN
    /*
    thread 'main' panicked at 'assertion failed: `(left == right)`
    left: `NaN`,
    right: `NaN`', src/main.rs:26:5
    */
    let x = (-42.0_f32).sqrt();
    //    assert_eq!(x, x);

    if x.is_nan(){
        println!("未定义的数学行为")
    }

    // 数学运算
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4  * 30;
    let quotient = 56.7 / 32.2;
    let remaider = 43 % 5;
    // 位运算
    let a:i32 = 2;
    let b:i32 = 3;
    println!("(a & b) value is {}", a & b);
    println!("(a | b) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {}", !b );
    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);
    let mut a = a;
    a <<=b;
    println!("a << b value is {}", a );

    // 序列， 只允许用于数字或字符类型
    for i in 'a'..'z' {
        println!("{}", i);
    }
    // 函数
    number_type();
}
