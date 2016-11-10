use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn main() {
    // The status is a mutable shared state. 0 means work, a 1 Party, all other values are unknown in this context.
    let status = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel::<i32>();

    // Worker Thread
    let (status, tx) = (status.clone(), tx.clone());
    thread::spawn(move || {
        loop {
            // we use `recv_timeout` and not `recv` to arive some non blocking behaveior
            match rx.recv_timeout(Duration::from_millis(100)) {
                Err(_) => {
                    println!("Work work work...");
                    thread::sleep(Duration::from_millis(500));
                }
                Ok(1) => {
                    break;
                }
                Ok(_) => {}
            }
        }
        println!("Feierabend! Schools Out! Rock 'n Roll!");
    });

    // This thread contolls how long they must work
    let (status, tx) = (status.clone(), tx.clone());
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(5000));
        let mut status = status.lock().unwrap();
        *status = 1;
        tx.send(*status).unwrap();
        println!("Buzz!");
    });

    // Main Loop
    loop {
    };
}
