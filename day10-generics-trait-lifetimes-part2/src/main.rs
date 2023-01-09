fn main() {
    println!("============ practice part 2 ==========");
    // 6

    let p1 = Point{x: 5, y:10};
    let p2 = Point{x: "haha", y:'中'};
    let p3 = p1.mixup(p2);
    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    //7

    let p4 = Point2::<f32>{x: 5.0, y:10.0};
    println!("{}", p4.distance_from_origin());

    // const generics
    // let arrays = [
    //     Array{
    //         data: [1, 2, 3],
    //     },
    //     Array {
    //         data: [1.0, 2.0, 3.0],
    //     },
    //     Array {
    //         data: [1, 2]
    //     }
    // ];

    
}

struct Point<T, U>{
    x:T,
    y:U,
}

impl <T, U> Point<T, U>{ // impl的TU用于限制结构体内部的字段
    fn mixup<T2, U2>(self, p: Point<T2, U2>)->Point<T, U2>{ // T2 U2限制参数中的范型
        Point { x: self.x, y: p.y }

    }
}


struct Point2<T>{
    x: T,
    y: T,
}

impl Point2<f32>{
    fn distance_from_origin(&self)-> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// const范型
struct Array<T, const N :usize> {
    data: [T; N]
}
