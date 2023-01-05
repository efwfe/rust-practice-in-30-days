/**
 * structure:
 *  定义和实例化结构体
 *  定义关联函数
 *  基于结构体和枚举创建新的类型
 *  
 * 结构体整个实例必须是可变的；rust并不允许某个字段为可变。
 * 使用没有字段名的远足结构体，叫做元组结构体。
 * 没有任何字段的结构体，叫类单元结构体。一般用于某个类型实现trait
 * 
 * 结构体可以创建有意义的自定义类型；在impl块中可以定义和函数相关联的方法
 */

 struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
 }


pub fn book() {
    let user1 = User{
        email: String::from("some@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count : 1,    
    };

    // structure update syntax
    let user2 = User{
        email: String::from("daihui@qq.com"),
        ..user1 // 放到最后 指定其余字段从user1中获取， user1的数据是move进去的

    };


    

}



pub fn practice(){
    struct Rectangle{
        width: u32,
        height:u32,
    };
    impl Rectangle{
        fn area(&self)->u32{
            self.width*self.height
        }
    }

    let rect1 = Rectangle{width:30, height:50};
    assert_eq!(rect1.area(), 1500);

    //2
    struct TrafficLight{
        color: String,
    }
    impl TrafficLight {
        pub fn show_state(&self){
            println!("the current state is {}",self.color);
        }
    }

    let light = TrafficLight{color: "red".to_owned()};
    light.show_state();

    // associated function
    #[derive(Debug)]
    struct TrafficLight2{
        color: String,
    }
    impl TrafficLight2{
        pub fn new()-> TrafficLight2{
            TrafficLight2 { color: "red".to_owned() }
        }

        pub fn get_state(&self)-> &str{
            &self.color
        }
    }

    let light2 = TrafficLight2::new();
    assert_eq!(light2.get_state(), "red");

    // enum method
    #[derive(Debug)]
    enum TrafficLightColor{
        Red,
        Yellow,
        Green,
    }

    impl TrafficLightColor{
        //todo
    }
    
}