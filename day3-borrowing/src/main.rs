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

    // refference and borrowing
    println!("referrence and borrowing");
    //1
    let x = 5;
    let p = &x;
    println!("x 的内存地址为：{:p}", p);

    //2
    let x = 5;
    let y = &x;
    assert_eq!(5,*y);

    //3 
    let mut s = String::from("hello5");
    borrow_object(&mut s);

    //4
    let mut s = String::from("hello,");
    push_str(&mut s);
    println!("4: {}", s);

    //5
    let mut s = String::from("hello 6");
    let p = &mut s;
    p.push_str("world");

    // ref
    // 6 
    let c = 'a';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));

    // 7
    let mut s = String::from("hello 7");
    let r1 = & s;
    let r2 = & s;
    println!("{}, {}", r1, r2);

    // 8
    let mut s = String::from("hello 8");
    borrow_object(&mut s);

    // 9
    let mut s = String::from("aaa");
    borrow2(&s);
    s.push_str(" aaa");
    println!("{}", s);
    //10
    let mut s = String::from("aaaa");
    let r1 = &mut s;
    r1.push_str("bbb");
    let r2 = &mut s;
    r2.push_str("ccc");
    println!("s:{}", s);

    //11
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    r1.push_str("aaa") // error

}
fn borrow2(s: &String){}

fn get_addr(r: &char)->String{
    format!("{:p}", r)
}

fn push_str(s: &mut String){
    s.push_str(" world");
}

fn borrow_object(s: &mut String){

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