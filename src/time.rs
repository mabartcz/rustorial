use std::time::{Duration, Instant};
extern crate chrono;

pub fn time_test(){

    let dur1 = Duration::from_secs(60);
    println!("{}", dur1.as_millis());

    let dur2 = Duration::from_millis(2000);

    let dur = dur1.checked_sub(dur2);
    println!("{}", dur.unwrap_or_default().as_millis());


    let time_now = Instant::now();

    std::thread::sleep(Duration::from_secs(3));

    println!("{:?}", time_now.elapsed().as_millis());

}


pub fn test_chrono() {
    let current_time = chrono::Utc::now();
    println!("{}", current_time.format("%Y %b %d"));
}