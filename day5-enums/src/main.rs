/**
 * 枚举和模式匹配
 * 枚举可以通过列举可能的成员（variants）来定义一个类型。
 * 特别的枚举Option，配合match表达式进行模式匹配；
 * if let 特殊的处理枚举的结构
 * 
 *  枚举
 * 
 *  结构体提供了一种将数据和字段结合的方式；
 *  枚举提供了一种更高程度的数据集合
 * 
 * 
 * Option枚举和其相对于空值的优势
 * 
 *  enum Option<T>{
 *      None,
 *      Some(T)
 * }
 * 
 * Match模式匹配，穷尽
 * if let 语法提供一种简单的结合方式
 * 当值匹配某一模式代码执行而忽略其他值
 * * 
 */
fn main() {
    day5::book();
    println!("===== practice ====");
    day5::practice();
}
