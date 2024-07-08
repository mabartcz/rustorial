pub fn test_vectors() {
    let mut data: Vec<i32> = Vec::new();

    data.push(1);
    data.push(2);

    println!("Size of vec: {}", data.len());
    println!("Capacity of vec: {}", data.capacity());

    data.push(3);
    data.push(4);
    data.push(5);

    println!("Size of vec: {}", data.len());
    println!("Capacity of vec: {}", data.capacity());

    println!("{}", data[0]);
    println!("{:?}", &data.as_slice()[1..]);

    println!("{:?}", data.get(10));
    let _number = data.get(10);
}

pub fn test_vec_string(){
    let mut  data = vec!["Hello"; 10];

    data.reserve(20);

    let mut data_2 = vec!["Ahoj"; 10];
    
    // Foorloop sežere data, musis loopnout přes clone
    for greet in data.clone() {
        println!("{} Marty !", greet);
    }

    println!("{:?}", data);
    data.append(&mut data_2);
    println!("{:?}", data);

    data.insert(0, "OlaOla");
    println!("{:?}", data);

    data.remove(0);
    println!("{:?}", data);

    data.retain(|e| {if *e == "Hello" {return true;} else {return false;}});
    println!("{:?}", data);
}