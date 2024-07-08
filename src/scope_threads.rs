use std::thread;

struct Person {
    name: String
}

pub fn test_thread_variables(){
    let age = 56;
    let p1 = Person{name: "Petra".to_string()};

    let print_age = || {
        println!("{age}");
        println!("{}", &p1.name);
    };

    thread::scope(|scope| {
        scope.spawn(print_age);
    });
    // thread::spawn(print_age).join().unwrap();

    println!("{age}");
    println!("{}", p1.name);
    
    println!("Finished ...")
}