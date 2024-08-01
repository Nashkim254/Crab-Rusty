use std::{thread, time::Duration};

pub fn run() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..5 {
        println!("number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

   
}

fn vectors(){
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let handle = thread::spawn(move || {
        println!("Here is a vector {:?}",  v);
    });
    // drop(v);  cannot call v since its been moved
    handle.join().unwrap();
}