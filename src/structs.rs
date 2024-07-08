use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
enum Color {
    Red,
    White,
    Blue
}


struct Car {
    wheels: Cell<u8>,
    name: String,
    color: Color
}

impl Car {
    // Instance method (changes instance of struct)
    fn paint(&mut self, color: Color) {
        self.color = color;
    }

    // Instance method is called with dot car.show
    // Statics is called :: Car::New_car()

    // Statics methode
    fn new() -> Car{ 
        let new_car = Car{
            wheels: Cell::from(0),
            name: String::from(""),
            color: Color::White
        };
        return new_car;
    }

    fn show(&self) {
        println!("Name: {0}, wheels: {1}, color {2:?}", self.name, self.wheels.get(), self.color);
    }
}


fn create_car() -> Car{
    let mut my_car = Car::new();

    my_car.wheels.set(4);
    my_car.name = "Křeček".to_string();

    my_car.paint(Color::White);

    return my_car;
}

 
pub fn test_structs(){
    let my_car = create_car();
    my_car.show();
}

//Tuple Struct

#[derive(Debug)]
#[allow(dead_code)]
struct Character(String, i32);

#[allow(dead_code)]
fn test_char() {
    let my_char: Character = Character("asd".to_string(), 568);
    println!("{:?}", my_char);
}