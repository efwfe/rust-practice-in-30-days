pub fn book(){
    enum IpAddrKind{
        V4,
        V6
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // enum  可以内嵌多种类型
    enum Message{
        Quit,
        Move {x: i32, y:i32},
        Write(String),
        ChangeColor(i32, i32,i32,i32),
    }

    let config_max = Some(3u8);
    match config_max{
        Some(max)=>{println!("Maxium is to be {}", max)},
        _ =>{}
    }

    if let Some(max) = config_max{
        println!("maxium is :{}", max);
    }

    let value = 5;
    match value{
        5 =>{println!("this is five")},
        _ =>{}
    }
    if let val = value{
        println!("value is {}", val);
    }
}


enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32,i32),
}


pub fn practice(){

    //1 
    enum Direction{
        East,
        West,
        North,
        South,
    }
    let dire = Direction::South;
    match dire{
        Direction::East => println!("East"),
        Direction::South | Direction::North=>{
            println!("South or North");
        },
        _ =>println!("West"),
    }

    // 2
    let boolean = true;
    let binary = match boolean{
        true => 1,
        false => 0,
    };
    assert_eq!(binary, 1);

    //3
  

    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255, 255, 255),
    ];
    for msg in msgs{
        show_message(msg);
    }

    //3
    let alphabets = ['a', 'e','z','0', 'x', '9','Y'];
    for ab in alphabets{
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }

    //4
    enum MyEnum{
        Foo,
        Bar
    }
    let mut count =0;
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v{
        if matches!(e, MyEnum::Foo){
            count +=1;
        }
    }
    assert_eq!(count, 2);

    //5
    let o = Some(7);
    if let Some(i) = o{
        println!("this is a really long string and `:{:?}`",i);
    }

    //6
    enum Foo{
        Bar(u8)
    }
    let a = Foo::Bar(1);
    if let Foo::Bar(x) = a{
        println!("foobar value is :{}",x);
    }

    //7
    enum Foo2{
        Bar,
        Baz,
        Qux(u32)

    }
    let a = Foo2::Qux(10);
    match(a){
        Foo2::Bar => println!("match foo::bar"),
        Foo2::Baz => println!("match foo:baz"),
        _=> println!("match others"),
    }

    // 8
    let age = Some(30);
    if let Some(age)= age{
        assert_eq!(age, 30);
    }
    match age {
        Some(age)=>println!(" age is new val, values is :{}", age),
        _=>{}
    }

}

fn show_message(msg: Message){
    match msg{
        Message::Move { x: a, y:b}=>{
            println!("a:{}, b:{}", a,b);
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        }
        Message::ChangeColor(_, g, b)=>{
            assert_eq!(g, 255);
            assert_eq!(b, 255);
        }
        _=> println!("no data in these variants")
    }
}