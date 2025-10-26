use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    safe_counter_with_mutex();
    counter_with_stop_condition();
}

// 1. safe: counter with mutex
fn safe_counter_with_mutex() {
    println!("2. Safe Counter with Arc<Mutex<T>>");

    // arc = atomic reference counter (multiple ownership)
    // mutex = mutual exclusion (thread-safe access)
    let counter = Arc::new(Mutex::new(0)); // Mutex<i32> this is value + mutex

    // thread 1: counts to 50
    let counter1 = Arc::clone(&counter); // necessary for ownership to go into thread
    let handle1 = thread::spawn(move || {
        for i in 0..50 {
            let mut num = counter1.lock().unwrap(); // lock for exclusive access
            *num += 1;
            println!("Thread 1 - Iteration {}: Counter = {}", i + 1, *num);
            drop(num); // release lock early
            thread::sleep(Duration::from_millis(10));
        }
    });

    // thread 2: counts to 50
    let counter2 = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        for i in 0..50 {
            let mut num = counter2.lock().unwrap(); // lock for exclusive access
            *num += 1;
            println!("Thread 2 - Iteration {}: Counter = {}", i + 1, *num);
            drop(num); // release lock early
            thread::sleep(Duration::from_millis(12));
        }
    });

    // wait for both threads
    handle1.join().unwrap();
    handle2.join().unwrap();

    let final_value = *counter.lock().unwrap();
    println!("✅ Final counter value: {} (should be 100)", final_value);
    println!("✅ No race condition - thanks to Mutex!\n");
}

// 3. counter with stop condition at 100
fn counter_with_stop_condition() {
    println!("3. Counter with Stop Condition at 100");

    let counter = Arc::new(Mutex::new(0));

    // thread 1: counts until counter >= 100
    let counter1 = Arc::clone(&counter);
    let handle1 = thread::spawn(move || {
        loop {
            let mut num = counter1.lock().unwrap();
            if *num >= 100 {
                println!("🛑 Thread 1: Counter reached 100, stopping!");
                break;
            }
            *num += 1;
            let current = *num;
            drop(num); // release lock

            println!("Thread 1: Counter = {}", current);
            thread::sleep(Duration::from_millis(8));
        }
    });

    // thread 2: counts until counter >= 100
    let counter2 = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        loop {
            let mut num = counter2.lock().unwrap();
            if *num >= 100 {
                println!("🛑 Thread 2: Counter reached 100, stopping!");
                break;
            }
            *num += 1;
            let current = *num;
            drop(num); // release lock

            println!("Thread 2: Counter = {}", current);
            thread::sleep(Duration::from_millis(10));
        }
    });

    // wait for both threads
    handle1.join().unwrap();
    handle2.join().unwrap();

    let final_value = *counter.lock().unwrap();
}
