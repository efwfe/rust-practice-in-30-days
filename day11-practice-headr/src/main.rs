/**
 * 1 创建可以选的命令行参数和值
 * 2 解析string为一个整数
 * 3 编写单元测试
 * 4 使用match语法
 * 5 使用From into as转换类型
 * 6 使用take获取一个迭代器或者一个文件描述符
 * 7 保存行末
 * 8 从文件中读取bytes
 * 8 使用turbofish操作符
 */

fn main() {
    let Err(e)= headr::get_args().and_then(headr::run){
        eprintln!("{}",e);
        std::process::exit(1);
    };
}
