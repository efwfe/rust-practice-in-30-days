use core::panic;
/**
 * 
 * rust 错误处理
 * 
 * rust将错误分为可恢复和不可恢复；
 * result 通常用于可恢复的错误；
 * panic！宏通常用于不可恢复；
 * 
 * 
 * match能够执行他的工作，但是比较冗长，有一些方法可以帮助我们
 * unwrap可以返回ok中的值，如果是err则会panic
 * expect() 也可以提供错误的跟踪
 * 
 * 错误处理的原则
 *  - 非预期的行为
 *  - 在此后代码的运行不处于有害状态
 *  - 没有可行的手段将有害的状态信息编码进所使用的类型中
 */

 use std::fs::File;
use std::num::ParseIntError;
use std::io::{self, Read};

fn main() {
    // let v = vec![1,2,3];
    // v[99];

    // let f = File::open("hello.txt");
    // let f = match f{
    //     Ok(file)=> file,
    //     Err(error)=>panic!("error happended"),
    // };

    // practive
    // 1
    // fn drink(beverage: &str){
    //     if beverage == "lemonade"{
    //         print!("success");
    //         panic!("exit");

    //     }
    //     println!("exercise failed");
    // }
    // drink(&"lemonade");

    //2
    let result = multiply("10", "20");
    assert_eq!(result, Ok(200));

    // let result = multiply("t", "2");
    // assert_eq!(result.unwrap_or(8), 8);
    

    //3 
    assert_eq!(multiply2("3", "4").unwrap(), 12);
    

    //4

    //5
    assert_eq!(add_two("4").unwrap(), 6);

}
fn add_two(n_str: &str)->Result<i32, ParseIntError>{
    n_str.parse::<i32>().map(|x| x+2)
}
fn read_file1()-> Result<String, io::Error>{
    let f = File::open("hello.txt");
    let mut f = match f{
        Ok(file)=>file,
        Err(e)=> return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s ){
        Ok(_)=> Ok(s),
        Err(e)=>Err(e),
    }
}

fn read_file2()->Result<String, io::Error>{
    let mut s = String::new();
    let mut f = File::open("hello.txt")?;
    match f.read_to_string(&mut s ){
        Ok(_)=> Ok(s),
        Err(e)=>Err(e),
    }
}
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>{
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn multiply2(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>{
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}