use std::sync::{mpsc};
use std::time::Duration;

pub fn test_channels(){
    let (sender, receiver) = mpsc::channel::<i32>();

    // sender.send(123);
    // sender.send(321);
    //
    // let msg = receiver.recv_timeout(Duration::from_millis(100));
    // println!("{:?}", msg);

    let get_data = move || {
      loop {
          println!("Recieve");
          let msg = receiver.recv_timeout(Duration::from_millis(500));
          println!("{:?}", msg);
      }
    };

    std::thread::spawn(get_data);

    
    

    for x in 1..10{
        println!("Send");
        sender.send(x);
        std::thread::sleep(Duration::from_secs(1));
    }


    // std::thread::spawn(get_data).join();


}