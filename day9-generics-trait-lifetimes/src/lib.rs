use std::fmt::Display;
use std::cmp::PartialOrd;

pub fn book(){
    let num_list = vec![12, 20, 30, 100];
    let result = largest(&num_list);
    println!("largest number is {}", result);


    let char_list = vec!['y', 'm', 'a','q'];
    let result = largest_char(&char_list);
    println!("char largest is {}", result);

    let result = largest(&char_list);
    println!("char largest is {}", result);

    // lifetime
    
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longer(&string1, &string2);
    // }
    // println!("longer one is {}", result);
}
// fn longer<'a,'b>(x:&'a String, y:&'b String)->&'a String{
//     if x.len() > y.len(){
//         x
//     }else{
//         y
//     }
// }

struct Point<T>{
    x: T,
    y: T,
}

enum Option<T>{
    Some(T),
    None,
}
// 单态化编译器处理
impl<T> Point<T>{ // impl后必须声明T，这样就可以在实现的方法中使用它了。
    fn x(&self)-> &T{
        &self.x
    }
}

// 定义trait
pub trait Summary{
    fn summarize(&self)->String;
    fn xxx(&self)->String;
}

// trait作为函数参数
pub fn notify(item: &impl Summary){
    println!("Breaking news: {}", item.summarize());
}

pub fn notify2<T:Summary>(item: &T){
    println!("news ;{}", item.summarize());
}

pub fn notify3<T:Summary+Display>(item: &T){
    println!("news ;{}", item.summarize());
    println!("{}", item);
}

pub fn some_function<T, U>(t: &T, u: &U)-> i32
    where T: Display + Clone,
          U: Clone + Summary,
          {
    1
}
// 作为函数返回值
// fn return_trait()->impl Summary{
    
// }
fn largest<T:PartialOrd +Copy>(list: &[T])-> T{
    let mut largest = list[0];
    for &item in list{
        if item> largest{
            largest = item;
        }
        
    }
    largest
}

fn largest_char(list: &[char])->char{
    let mut largest = list[0];
    for &item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}


fn largest_num(list: &[i32])->i32{
    let mut largest = list[0];
    for &item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}


// ============================================

pub fn practice(){

    //1

// 填空
struct A;          // 具体的类型 `A`.
struct S(A);       // 具体的类型 `S`.
struct SGen<T>(T); // 泛型 `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}
    // 使用非泛型函数
reg_fn(S(A));          // 具体的类型
gen_spec_t(SGen(A));   // 隐式地指定类型参数  `A`.
gen_spec_i32(SGen(6)); // 隐式地指定类型参数`i32`.

// 显式地指定类型参数 `char`
generic::<char>(SGen(('a')));

// 隐式地指定类型参数 `char`.
generic(SGen(('c')));


// 2
assert_eq!(5, sum(2i8, 3i8));
//3
let integer = Point2{x:1, y:2};
let float = Point2{x:2.1, y:21.1};

//4
let p = Point3{x:5, y:"hello"};

//5
let x = Val{val: 3.0};
let y = Val{val: "hello".to_string()};
println!("{}, {}", x.value(), y.value());


}

struct Val<T>{
    val: T,
}

impl<T> Val<T>{
    fn value(&self)-> &T{
        &self.val
    }
}


fn sum<T:std::ops::Add<Output = T>>(x:T, y:T)->T {
    x+y
}

struct Point2<T>{
    x:T,
    y:T,
}

struct Point3<T,U>{
    x:T,
    y:U,
}