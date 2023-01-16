use std::io::Seek;

/**
 * 面向对象语言
 *  封装 继承 多态
 * rust：封装， trait特性分发
 */
use day16::Draw;
use day16::{Button, Screen};

struct SelectBox{
    width:u32,
    height:u32,
    options:Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self) {
        println!("select box drawing");
    }
}
fn main() {
    let screen  = Screen{
        components:vec![
            Box::new(SelectBox{
                width:12,
                height:10,
                options:vec![
                    String::from("yes"),
                    String::from("No"),
                ]
            }),
            Box::new(Button{
                width:300,
                height:100,
                label: String::from("Ok"),
            })
        ]
    };
    screen.run()
}
