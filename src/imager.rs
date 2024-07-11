use image::GenericImageView;

pub fn test_image() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("c:/Users/bartonm/Desktop/test_img.png").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    // img.save("test.png").unwrap();
    
    let small_img = img.resize(1280, 720, image::imageops::FilterType::Nearest);
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", small_img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", small_img.color());

}