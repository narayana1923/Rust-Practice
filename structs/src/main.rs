#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}
// we can also make them in a single block
impl Rectangle {
    /*
     * This is not a method as it doesn't has first parameter as self.
     * In rust, this is called as an associated function and needs to be called using
     * :: (Scope resolution, atleast thats what they say this in c++ ) operation
     */
    fn create_square(size: i32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 22,
        width: 1,
    };

    let rect_area = rect.area();
    println!(
        "Area of rectangle with height {} and width {} is {rect_area}",
        rect.height, rect.width
    );
    // Debug format
    println!("Area of {:?} is {rect_area}", rect);
    println!("\nArea of \n{:#?}\nis {rect_area}", rect);
    dbg!(&rect);

    // rect.create_square() // Does not work as this is an associated function
    let ar = Rectangle::area(&rect); // This works
    dbg!(ar);
    // May be like static functions in java but can only be called in this way....
    let square = Rectangle::create_square(4);
    // dbg!(square); // This will throw error as value is moved
    dbg!(&square);
    println!("Area of square is {}", square.area());
}
