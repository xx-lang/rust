#[warn(unused_variables)]
mod cli;

use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::sync::{Arc, Mutex};
use std::thread;
// use std::rc::Rc;
use cli::Cli;

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

fn main() {
    use_ferris_says();
    use_struct();
    change_static_value();
    use_thread_mutex();
    use_cli();
}

fn use_ferris_says() {
    let stdout = stdout();
    let out = b"Hello World";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}

fn use_struct() {
    let rect = Rectangle {
        width: 10,
        height: 10,
    };
    println!("rect {:?} area is {}", rect, rect.area());
}

fn change_static_value() {
    static mut G1: u8 = 10;
    // println!("global value {}", G1); // must in unsafe body
    unsafe {
        G1 = 20;
        println!("global value changed {}", G1);
    }
}

fn use_cli() {
    let inst = Cli { args: 1 };
    inst.say();
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
