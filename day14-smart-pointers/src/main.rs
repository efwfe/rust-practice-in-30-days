

/**
 * 指针，包含内存地址的变量的通用概念。reference引用以&借用所指向的值
 *  smart pointer是一类数据结构，表现类似指针，也拥有额外元数据和功能。
 * 普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针，大部分情况下，智能指针拥有他们指向的数据。
 * 
 * - Box<T> 堆上分配
 * - Rc<T>  一个引用计数类型，数据可以有多个所有者
 * - Ref<T> RefMut<T> 通过RefCell<T> 访问
 * 
 * 内部可变性模式，不可变类型暴露出改变其内部值的API
 * 
 * Deref强制转换只能作用于实现了Deref trai的类型。可以将一个类型的引用转换为另一个类型的引用。
 */
use std::rc::Rc;
use crate::List2::{Cons, Nil};

// enum List{
//     Cons(i32, Box<List>),
//     Nil,
// }
struct  CustomSmartPointer{
    data :String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data:{}", self.data);
    }

}

enum List2{
    Cons(i32, Rc<List2>),
    Nil,
}



fn main() {
    // use crate::List::{Cons, Nil};
    // let b = Box::new(5);
    // println!(" b = {} ", b);

    // Box允许创建递归类型
    // let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    // let x = 5;
    // let z = Box::new(x);
    // let y = &x;
    // assert_eq!(*y, 5);
    // assert_eq!(x,5);
    // assert_eq!(*z, 5);

    // Deref do 
    let m = Box::new(String::from("Rust"));
    hello(&(*m)[..]);

    //Drop Trait清理代码
    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };
    // 提前drop
    // c.drop();// error 这会导致double free。
    let d = CustomSmartPointer{
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created!");

    
    // Rc
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // RefCell和内部可变性
    // Rc允许数据有多个所有者，Refcell允许在运行时执行不可变或者可变检查

}

fn hello(str: &str){
    println!("{}", str);
}