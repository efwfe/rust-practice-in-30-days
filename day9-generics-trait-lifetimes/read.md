范型是具体类型或其他属性的抽象替代，

- generics

1. 函数的范型

```rust
fn largets<T>(&list: &[T])->T{
    ...
}
```

2. 结构体范型

```rust

struct Point<T>{
    x:T,
    y:T
}

```

3. 方法的范型

```rust
impl <T>Point<T>{
    fn x(&self)->&T{
        &self.x
    }
}

```

- trait

trait 告诉 rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以使用 trait bounds 指定范型是任何拥有特定行为的类型。

trait 定义是一种将方法签名组合起来的方法，定义一个实现某些目的所必需的行为的集合。

trait 必须和类型一起引入作用域以便使用额外的 trait 方法。
trait 某些提供默认行为，当为某个类型提供特定实现 trait 时，可以选择保留或者重载每个方法的默认实现。

- lifetime

  rust 每一个引用都有生命周期，引用保持有效的作用域。
  borrow checker
  生命周期参数注解位于引用的&之后，

结构体生命周期注解

```rust
struct ImportantExcerpt<'a>{
    part: &'a str,

}
```

'static 生命周期存在于整个程序期间，所有的字符串字面量都是'static 生命周期。
