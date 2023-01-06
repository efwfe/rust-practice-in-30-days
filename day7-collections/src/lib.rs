
pub mod libs{
    pub fn book(){
        // ========= Vector 存储列表 ============
        let v: Vec<i32> = Vec::new();
        // 类型注解的目的，rust并不知道我们要存储什么类型的元素，vector是范型实现
        let v = vec![1,2,3]; // vec!宏

        // vec元素的引用
        let v = vec![1,2,3,4,5];
        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(2){
            Some(third)=>println!("The third element is {}", third),
            None=>println!("no third element"),
        }
        
        // 便利vector中的元素
        let v = vec![100, 32, 57];
        for i in &v{
            println!("{}", i);
        }

        // vec只能存储相同类型 使用枚举来存储多种类型
        // 如果在编写程序的时候不能确切的知道运行时存储到vec所有类型，枚举技术就不行，
        // 可以使用trait

        // =========== String ================
        // 字符串比较复杂的数据结构，由于字符串的字面量被存储在二进制输出中
         // 字符串字面量也是字符串slices

         /** https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str
          * String is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.
          * str is an immutable1 sequence of UTF-8 bytes of dynamic length somewhere in memory. Since the size is unknown, one can only handle it behind a pointer. This means that str most commonly2 appears as &str: a reference to some UTF-8 data, normally called a "string slice" or just a "slice". A slice is just a view onto some data, and that data can be anywhere, e.g.
          */

        let mut s = String::new();
        let data = "initial contents";
        let s = data.to_string();
        let s = "initial contents".to_string();
        //
        let mut s = "hello".to_string();
        s.push_str("world");
        println!("s is {}", s);

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s =format!("{}-{}-{}", s1, s2, s3);
        println!("{}",s);

        //String是一个Vec<u8>的封装，由于utf8字节数不固定，不允许根据索引获取char
        // 索引字符串返回的类型是不明确的：字节值，字符，字符串slice

        // 明确表示需要字符
        for c in "asda".chars(){
            println!("{}", c);
        }

        for b in "hello".bytes(){
            println!("{}", b);
        }

        // 哈希map
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        // hashmap的k的类型需要保持一致，value的类型也需要保持一致，如果将值的引用插入hashmap，这些值本书将不会移动进hashmap，
        // 但这些引用指向的值至少在哈希map有效时也是有效的
        for (key, value) in &scores{
            println!("{}: {}", key, value);
        }

        // 只有键没有值时插入
        scores.entry(String::from("Yellow")).or_insert(50);


        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace(){
            let count = map.entry(word).or_insert(0);
            *count +=1;
        }

        println!("{:?}", map);

    }

    pub fn practice(){

        // 1 
        let mut s : String = "hello, ".to_owned();
        s.push_str("world");
        s.push('!');

        move_ownership(s.clone());

        assert_eq!(s, "hello, world!");
        println!("success")
        

    }
    fn move_ownership(s: String){
    println!("ownership of {} is moved here.", s);

    // 2 

    let mut s = String::from("hello, world");
    let slice1: &str = &s[..];
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[..5];
    assert_eq!(slice2, "hello");

    let slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    //3 2次堆内存
    

    let mut s = String::new();
    println!("{}", s.capacity());
    for _ in 0..2{
        s.push_str("hello");
        println!("{}", s.capacity());
    }


    use std::mem;
    let story = String::from("Rust by Practice");
    // 阻止内存释放
    let mut story = mem::ManuallyDrop::new(story);
    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();
    assert_eq!(16, len);
    let s = unsafe{String::from_raw_parts(ptr, len, capacity)};
    assert_eq!(*story, s);
    println!("success");
}
}

