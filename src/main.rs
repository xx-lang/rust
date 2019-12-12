use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::sync::{Arc, Mutex};
use std::thread;
// use std::rc::Rc;

fn main() {
    let stdout = stdout();
    let out = b"Hello World";
    let width = 24;

    static mut G1: u8 = 10;
    // println!("global value {}", G1); // must in unsafe body
    unsafe {
        G1 = 20;
        println!("global value changed {}", G1);
    }

    // let x : i32 = 9;
    // println!("9 pow 3 = {}", x.pow(3));

    let x = 1;
    let mut y = 2;
    // 注意这里专门用括号括起来了
    let z = (y = x);
    println!("{:?}", z);

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle {
        width: 10,
        height: 10,
    };
    println!("rect {:?} area is {}", rect, rect.area());

    // mutex
    use_thread_mutex();
}

fn use_thread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}
