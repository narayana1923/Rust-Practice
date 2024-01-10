fn main() {
    // will throw error if annotation is not used as
    // resultant data can be any type like u32, i32, etc
    let num: f32 = "42.3".parse().expect("Not a number");
    println!("num is {num}");

    /*
     * Two types of data:
     * 1. Scalar: represent single value like boolean, int, etc
     * 2. Compound: holds multiple values
     */

    // Scalar types
    /*
     * integer types: i8, u8, i16, u16,...., i128, u128, isize, usize
     */
    // isize will depend on the architecture
    // on 32 bit machine it will be i32 and i64 on 64 bit machines
    let num1: isize = 32;
    println!("Num1 value is {num1}");

    // Compound types: array, tuples
    // tuple: hold multiple types of data and cannot grow

    // works even without type annotation
    let tup: (i32, f32, i8) = (500, 6.4, 1);
    // println!("Tuple is {tup}"); // doesn't work

    // tuple destructuring... underscore makes compiler to ignore unused variables
    let (x, y, z) = tup;
    println!("tuple value is ({x}, {y}, {z})");

    // Accessing using index.. weird but works
    let second_index_val = tup.2;
    println!("Second index value of tuple is {second_index_val}");
}
