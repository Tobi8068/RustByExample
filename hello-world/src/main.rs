// #[derive(Debug)]
// struct Person<'a> {
//     name: &'a str,
//     age: u8,
// }

use std::fmt::{self, write};

struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(({}, {}))", self.0, self.1)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}
fn main() {
//     println!("{}, Hello, world!", 32);
//     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
//     println!("{number:>5}", number = 1);
//     println!("{number:0>width$}", number = 1, width = 5);
//     println!("My name is {0}, {1} {0}", "Bond", "Alex");
//     println!("My name is {0}, {1} {0}", "Bond", "Bob");
//     println!("{:?} months in a year.", 12);
//     let number: f64 = 1.0;
//     let width: usize = 5;
//     println!("{number:>width$}");

//     let name = "Peter";
//     let age = 27;
//     let peter = Person { name, age };

//     println!("{:#?}", peter.name);

    // let minmax = MinMax(0, 14);
    // println!("{}", minmax);    
}

