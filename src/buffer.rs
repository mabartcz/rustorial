use circular_buffer::CircularBuffer;

pub fn test_buffer() {
    let mut buf = CircularBuffer::<3, u32>::new();
    
    // Add a few elements
    buf.push_back(1);
    buf.push_back(2);
    buf.push_back(3);
    buf.push_back(4);

    
    println!("{:?}", buf);

    println!("{:?}", buf.pop_front());

    println!("{:?}", buf);

    println!("{:?}", buf.pop_front());

    println!("{:?}", buf);

    println!("{:?}", buf.pop_front());
    println!("{:?}", buf);
    println!("{:?}", buf.pop_front());

}