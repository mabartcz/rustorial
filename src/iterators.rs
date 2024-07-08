pub fn test_iterators(){

    let data = vec!["Red", "Blue", "Orange", "Yellow"];

    println!("{:?}", data);

    let mut data_iter = data.iter();

    // for color in data_iter {
    //     println!("{}", color);
    // }

    data_iter.next();

    let item1 = data_iter.next();
    println!("{:?}", item1);

    let data2 = vec!["Apple", "Banana", "Nuts"];

    // .chain() iterate one after other
    let all_data = data.iter().chain(data2.iter());

    // for x in all_data {
    //     println!("{:?}", x)
    // }

    let all_data_collection: Vec<&&str> = all_data.collect();
    println!("{:?}", all_data_collection);

    let new_data = data.iter().map(|x| String::from(*x));
    let data_edited = new_data.map(|mut x| {x.push_str(" !"); x});

    // For loop with closure for iterator
    data_edited.clone().for_each(|x| println!("{}", x));

    println!("Lase: {:?}", data_edited.clone().last().unwrap());
    println!("Skipped: {:?}", data_edited.clone().skip(1));

    let toto_vec = ["Boots", "Sun", "Moon", "Clouds"];
    let mut toto = toto_vec.iter().step_by(2);

    println!("next - {:?}", toto.next());
    println!("peek - {:?}", toto.clone().peekable().peek());
    println!("next - {:?}", toto.next());
    println!("next - {:?}", toto.next());

    // enumerate
    for (index, value) in toto_vec.iter().enumerate(){
        println!("{0} - {1}", index, value);
    }



}

