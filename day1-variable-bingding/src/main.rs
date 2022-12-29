#[warn(unused_variables)]

fn main() {
    // 修复下面代码的错误并尽可能少的修改
    // let x: i32; // 未初始化，但被使用
    // let y: i32; // 未初始化，也未被使用
    // println!("x is equal to {}", x); 


    let mut x = 1;
    x += 2;
    println!("x = {}", x);

    // 变量作用域
     let x: i32 = 10;
     let y: i32;
    {
        y = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}, y 的值是 {}", x, y); 

    println!("{}, world!", define_x());

    // Shadowing
    let x:i32=5;
    {
        let x = 12;
        assert_eq!(x,12);
    }
    assert_eq!(x, 5);
    let x = 42;
    println!("{}", x);

    // 未使用的变量
    let x = 1;
    let _x =1;
    // 变量解构
    let (mut x, mut y) = (1, 2);
    x +=2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    // 解构赋值
    let (x, y);
    (x,..) = (3, 4);
    [..,y] = [1, 2];
    assert_eq!([x, y], [3, 2]);
    
}


fn define_x()->String{
    String::from("hello")
}