use std::thread;
use std::time::Duration;
// mpsc stands for multiple producer, single consumer
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    // _spawned_thread_stops_prematurely();
    // _fixed_thread_stops_as_expected();
    // _send_single_message();
    // _send_mult_messages();
    // _multiple_producer();
    // _mutex_single_thread();
    mutex_multi_thread_with_clone();
}

fn mutex_multi_thread_with_clone() {
    // the Arc smart pointer uses the Send trait which let's the counter communicate with the Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut number = counter.lock().unwrap();

            *number += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// fn mutex_multi_thread_clone_fix_wrong_kind_of_clone() {
//     // Rc is a smart pointer
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         // cloning is the right idea, but Rc does not work because Rc does not use the send trait:
//         // Send is "one of the traits that ensures the types we use with threads are meant for use in concurrent"
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// fn mutex_multi_thread_naive_approach() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             // can only move the counter once, the iterator moves it multiple times
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// Using Mutexes to Allow Access to Data from One Thread at a Time
// Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock. The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. Therefore, the mutex is described as guarding the data it holds via the locking system.
// Mutexes have a reputation for being difficult to use because you have to remember two rules:
// - You must attempt to acquire the lock before using the data.
// - When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
fn _mutex_single_thread() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // when the scope ends here the mutex lock get dropped automatically

    println!("m = {:?}", m);
}

fn _multiple_producer() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

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

    for received in rx {
        println!("Got: {}", received);
    }
}

fn _send_mult_messages() {
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

    for received in rx {
        println!("Got: {}", received);
    }
}

fn _send_single_message() {
    // transmitter and reciever
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn _spawned_thread_stops_prematurely() {
    thread::spawn(|| {
        // spawned thread goes to 10.
        // but since main thread only goes to 5 the program will end before this can finish its loop
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // main thread goes to 5
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

fn _fixed_thread_stops_as_expected() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if the join is moved here the program will run in a synchronous fashion
    // ie - loop will complete before main thread loop begins.
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // saving the thread to a handle and running here allows it to continue running after the main thread has completed
    // calling join on the handle "blocks" the thread currently running until the thread represented by the handle terminates.
    handle.join().unwrap();
}
