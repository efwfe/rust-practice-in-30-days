use std::iter::Rev;
/**
 * Concurrent Programming
 */
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};


fn main() {
    // ======= 创建线程来同时运行多段代码 ======
    // 1. spawn创建新线程

    // let handle = thread::spawn(||{
    //     for i in 1..10{
    //         println!("hi number {} from the spawned thead!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();

    // 2.线程与move闭包

    // let v = vec![1,2,3];
    // let handle = thread::spawn(move ||{
    //     println!("here's a vector :{:?}", v);
    // });
    // handle.join().unwrap();

    // =======  消息传递并发，channel被用来线程间 传递消息 ======= 

    // let (tx, rx) = mpsc::channel(); // transfor, recevicer
    // let tx1 = tx.clone();
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });
    // let received = rx.recv().unwrap();
    // println!("Got:{}", received);

    // thread::spawn(move|| {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     for val in vals{
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // for received in rx{
    //     println!("Got: {}", received);
    // }

    // =======  共享状态并发 =======
    /**
     * 互斥器 mutex, mutual exclusion的缩写。
     * 只允许一个线程访问某些数据。
     */
    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join();
    }
    println!("Result:{} ", *counter.lock().unwrap());

    // =======  sync 和 send trait将并发保证拓展到用户定义的以及标准库提供的类型中 =======
    /**
     * 通过Send允许在线程间转移所有权
     * Sync的类型可以安全的再多个线程中拥有其值的引用
     */
}
