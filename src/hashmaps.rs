use std::collections::{HashMap, HashSet};

pub fn test_hashmap(){

    // hashmap is "dictionary" - key/value pairs

    let mut stocks: HashMap<String, f32> = HashMap::new();

    stocks.insert("NVDA".to_string(), 485.2);
    stocks.insert("AAPL2".to_string(), 325.6);
    stocks.insert("AAPL".to_string(), 325.6);

    println!("{:?}", stocks.is_empty());
    println!("{:?}", stocks.len());

    println!("{:?}", stocks);

    stocks.remove(&("AAPL2".to_string()));
    println!("{:?}", stocks);
    
    // If you don't want to overwrite
    stocks.entry("META".to_string()).or_insert(444.0);
    stocks.entry("META".to_string()).or_insert(400.0);

    
    for (key, value) in stocks{
        println!("{} - {}", key, value);
    }
}


pub fn test_hashset() {
    // Set of unique values
    let mut data = HashSet::from(["Earth", "Mercur", "Earth"]);

    println!("{:?}", data);
    
    data.insert("Pluto");
    println!("{:?}", data);

    data.insert("Pluto");
    println!("{:?}", data);
}