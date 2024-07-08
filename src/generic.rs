use std::error;

#[derive(Debug)]
struct Person<T: Os>{
    name: String,
    system: T
}

trait Os{
    fn sound(&self) -> Result<String, Box<dyn error::Error>>;
}


struct Linux{}
impl Os for Linux{
    fn sound(&self) -> Result<String, Box<dyn error::Error>> {
        println!("Lixxx, Lixxx");
        return Ok("Pepa".to_string());
    }
}

// struct Windows{}
// impl Os for Windows{
//     fn sound(&self) -> () {
//         println!("Win, Win")
//     }
// }

// struct Macos{}
// impl Os for Macos{
//     fn sound(&self) -> () {
//         println!("Mac, ossss")
//     }
// }

pub fn test_generic() {
    let d1 = Linux{};
    let p1 = Person{name: "Marty".to_string(), system: d1};

    let a = p1.system.sound();

    let data = match a {
        Ok(data) => data,
        Err(_) => panic!()
    };

    println!("{:?}", p1.name);
    println!("{:?}", data);
}