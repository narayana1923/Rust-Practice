fn main() {
    let var1 = 5; // immutable by default
    println!("var1 value is {var1}");
    let var1 = "hello";
    println!("var1 value is {var1} now and it is a string");

    let var2 = 25;
    println!("Var2 value is {var2}");
    let mut var2 = 13;
    println!("Var2 value is {var2} and is mutable now");
    var2 = var2 - 5;
    println!("Var2 value after modifying is {var2}");

    // var2="hello"; // this will throw error
    // let var2="hello"; // this works

    let var3 = 5;
    println!("var3 initial value is {var3}");
    let var3 = var3 + 10;
    println!("var3 updated value is {var3}");

    {
        let mut var3 = var3 + 25;
        var3 += 24;
        println!("Var3 value in inner scope is {var3}")
    }
    // var3+=10; // this will throw an error now as immutable var3 is only shadowed inside the curly braces
    println!("Var3 value in outer scope");
}
