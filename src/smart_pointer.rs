enum List {
    Cons(i32, Box<List>),
    Nil
}


use crate::smart_pointer::List::{Cons,Nil};

pub fn run() {
    let name = Box::new("Imran Sheikh");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("Name: {name}");
}