use std::{sync::mpsc, thread, time::Duration};

pub fn run() {
    spawn_thread();
    vectors();
    message_passing();
    message_passing_with_timer();
}
fn spawn_thread() {
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
fn vectors() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let handle = thread::spawn(move || {
        println!("Here is a vector {:?}", v);
    });
    // drop(v);  cannot call v since its been moved
    handle.join().unwrap();
}
fn message_passing() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("Hello world");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received message {}", received);
}

fn message_passing_with_timer() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello world"),
            String::from("Hello world"),
            String::from("Hello world"),
            String::from("Hello world"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("youuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received message {}", received);
    }
}
