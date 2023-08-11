use std::thread;
use std::time::Duration;

fn main() {
    fixed_thread_stops_as_expected();
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

fn fixed_thread_stops_as_expected() {
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
