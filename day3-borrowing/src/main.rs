fn main() {
    let x = String::from("hello");
    let y = x.clone();
    println!("{}, {}", x, y);

    //2 
    let s1 = String::from("hello world");
    let s2 = take_ownershop(s1);
    println!("{}", s2);

    //3 
    let s = give_ownership();
    println!("{}", s);

    //4 
    let s = String::from("hello 4");
    print_str(&s);
    println!("{}", s);

    //5
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);

    //6 可变形
    let s = String::from("hello 5");
    let mut s1 = s;
    s1.push_str("world");
    println!("{}", s1);

    //7 
    let x = Box::new(5);
    let mut y = x.clone();
    *y = 4;
    assert_eq!(*x, 5);

    // 8
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    println!("{:?}", t.1);

    //9
    let t = (String::from("hello"), String::from("world"));
    let (ref x, ref y) = t;
    println!("{:?}, {:?}, {:?}", s1, s2, t);
}

fn print_str(s:&String){
    println!("{}", s);
}

fn take_ownershop(s:String)->String{
    println!("{}", s);
    s
}

fn give_ownership()->String{
    let s = String::from("hello 3");
    let _s = s.clone().into_bytes();
    s
}