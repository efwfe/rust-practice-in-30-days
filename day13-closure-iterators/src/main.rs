/**
 * 
 * 闭包，一个可以存储在变量里的类似函数的结构，可以保存在变量中传递给其他函数的匿名函数
 *      不需要类型注解，因为它不像是函数暴露在外的接口，他们存储在变量中并被使用，不用命名或者暴露给库的用户调用
 *      闭包定义会为每个参数和返回值推断一个具体的类型。
 * 
 *      闭包可以通过三种方式捕获值 Fn trait：
 *          1. FnOnce 消费从周围作用域捕获的变量，必须获得其所有权并在定义闭包时将其移动进闭包；
 *          2. FnMut 获取可变的借用值所以可以改变其环境
 *          3. Fn从环境获取不可变的借用值
 * 
 *      如果强制闭包获取所有权可以在参数列表前使用move关键字
 * 
 * 迭代器，一种处理元素序列的方式，
 *  迭代器都实现了Iterator trait
 * pub trait Iterator{
 *      type Item;
 *      fn next(&mut self)->Option<Self::Item>;
 * }
 *  
 */

fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("data"));
    // let t = example_closure(2);// error expected struct String

    let v1: Vec<i32> = vec![1,2,3];
    let dat: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("{:?}", dat);

    #[test]
    fn iterator_demonstration(){
        let v1 = vec![1,2,3];
        let mut v1_iter = v1.iter();//生成可变的迭代器，迭代器不可变是不可能的吧？mut 因为next方法改变了迭代器记录序列位置的状态
        assert_eq!(v1_iter.next(), Some(&1));// next返回值是不可变引用
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

}
