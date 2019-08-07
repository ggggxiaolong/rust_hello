fn main() {
    // thread_loop();
    // channel::mutipul_sender_message();
    lock::add();
}

mod channel {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    fn thread_loop() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();
    }

    fn thread_channel() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        let recived = rx.recv().unwrap();
        println!("Got : {}", recived);
    }

    fn mutipul_message() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for recive in rx {
            println!("Got: {}", recive);
        }
    }

    pub fn mutipul_sender_message() {
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for message in rx {
            println!("Got: {}", message);
        }
    }
}

mod lock {
    use std::sync::{Mutex,Arc};
    use std::rc::Rc;
    use std::thread;

    pub fn use_mutex(){
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    pub fn add(){
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec!();

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handler);
        }

        for handler in handles {
            handler.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}