use std::thread::{sleep, spawn};


fn do_math(workload: u128)  -> u128{
    let mut x: u128 = 0;

    for i in 1..workload {
        x += i;
    }
    x
}


pub fn test_threading() {

    let thread_fn = |x: u128| {
        let result = do_math(x);
        println!("Result: {result}")
    };

    // println!("Start ...");
    // let handler = spawn(thread_fn);
    // handler.join();
    // println!("... End");

    let handler1 = spawn(move || thread_fn(200_000_000));
    let handler2 = spawn(move || thread_fn(600_000_000));

    loop{
        sleep(std::time::Duration::from_secs(1));
        println!("**********************************");
        println!("Thread 1 status: {}", handler1.is_finished());
        println!("Thread 2 status: {}", handler2.is_finished());

        if handler1.is_finished() && handler2.is_finished(){
            break;
        }
    }

    // Join the threads to ensure they complete
    handler1.join().unwrap();
    handler2.join().unwrap();
}

