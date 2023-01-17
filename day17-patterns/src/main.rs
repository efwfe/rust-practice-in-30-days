/**
 * 模式是Rust中特殊的语法，用来匹配类型中的结构。
 * 模式匹配有以下部分组成：
 *  - 字面量
 *  - 解构的数组，枚举，结构体或者元祖
 *  - 变量
 *  - 通配符
 *  - 占位符
 * 
 */
fn main() {
     // if let
     let color:Option<&str>=None;
     let age:Result<u8,_>= "34".parse();
     if let Some(color)=color{
        println!("color: {}", color);
     }else if let Ok(age) = age{
        println!("age is {}", age);
     }else{
        println!("age parse error.");
     }
     //for loop
     let v = vec![1,2,3];
     for(index, value) in v.iter().enumerate(){
        println!("{} - {}", index, value);
     }
     // let
     let (x, y, z)=(1,2,3);

     // 函数参数
     let z = (1, 2);
     print_coodinates(&z);

     // ==========  match =================
     // 字面量
     let x = 1;
     match x{
        1=>println!("one"),
        2=>println!("two"),
        3=>println!("three"),
        _=>println!("nothing"),
     }
     // 命名变量
     let x= Some(5);
     let y = 10;
     match x{
        Some(50)=>println!("got 50"),
        Some(y)=>println!("Matched, y={y}"),
        _=>println!("Default case")
     }

     // 多个模式
     let x=1;
     match x{
        1|2=>println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
     }

     // 匹配值的范围
     let x = 5;
     match x {
        1..=5 => println!("one through fiv"),
        _=>println!("something else"),
     }

     l
}

fn print_coodinates(&(x, y): &(i32, i32)){
    println!("{}, {}", x, y);
}
