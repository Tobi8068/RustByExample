fn main() {
    println!("{}, Hello, world!", 32);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{number:>5}", number=1);
    println!("{number:0>width$}", number=1, width=5);
    println!("My name is {0}, {1} {0}", "Bond", "Alex");
    println!("My name is {0}, {1} {0}", "Bond", "Bob");
    println!("{:?} months in a year.", 12);
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}