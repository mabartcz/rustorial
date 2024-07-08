use std::ops::AddAssign;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn text_mutex_basic() {
    let score = Mutex::new(0);
    println!("{:?}", score);
    score.lock().unwrap().add_assign(5);
    println!("{:?}", score);
}



pub fn text_mutex() {
    let score = Mutex::new(0);

    let myfcn = || {
        println!("1- Started ...");

        loop {
            let guard = score.try_lock();

            if let Ok(mut data) = guard{
                for i in 1..10{
                    data.add_assign(i);
                }
                println!("1- ... Stopped");
                break;
            }
            else {
                thread::sleep(Duration::from_millis(300));
                println!("Waiting for lock.");
            }
        }
        // drop(data);
        // panic!("1- ERR");
    };

    let myfcn2 = || {
        println!("2- Started ...");
        let mut data = score.lock().unwrap();
        for i in 1..10{
            data.add_assign(i);
            thread::sleep(Duration::from_millis(100));
        }
        println!("2- ... Stopped");
    };

    thread::scope(|s|{

        s.spawn(myfcn2);
        s.spawn(myfcn);

        // let h1 = s.spawn(myfcn).join();
        // let h2 = s.spawn(myfcn2).join();

        // if h1.is_err(){
        //     println!("Error in thr 1.")
        // }
    });

    println!("{:?}", score);
}