
pub  fn number_type(){
    let x : i32 = 5;
    let mut y: u32 = 5;
    y = x as u32;
    let z = 10; // z i32
    println!("{}", type_of(&z));

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    let v1 = 251_u32 + 8;
    let v2 = i32::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);

    // let v = 1_024 as i64 + 0xff as i64 + 0o77 as i64 + 0b1111_1111 as i64;
    // assert!(v == 1579);
    
}


fn type_of<T>(_: &T)->String{
    format!("{}", std::any::type_name::<T>())
}