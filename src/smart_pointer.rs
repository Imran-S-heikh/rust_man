#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::{ops::Deref, rc::Rc};

use crate::smart_pointer::List::{Cons, Nil};
enum RList {
    Cons(i32,Rc<RList>),
    Nil
}

use crate::smart_pointer::RList::{Cons as RcCons,Nil as RcNil};


struct MyBox<T>(T);

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self)-> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct CustomPointer {
    data: String
}

impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("Dropping custom pointer with data: {}",self.data)
    }
}

pub fn run() {
    {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        println!("{:?}", list);
    }

    {
        let num = 5;
        let num = &num;

        assert_eq!(5, *num);
    }

    {
        let num = 5;
        let num = Box::new(num);

        assert_eq!(5, *num);
    }


    {
        let num = 34;
        let num = MyBox::new(num);

        assert_eq!(34,*num)
    
    }


    {
        let name = CustomPointer{
            data: String::from("Imran Sheikh")
        };

        let age = CustomPointer {
            data: String::from("Twenty Two")
        };
        
        drop(name);

        println!("Custom Pointers Created")
    }

    {
        let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
        let b = RcCons(3, Rc::clone(&a));
        let c = RcCons(4,Rc::clone(&a));
    }

    {
        let mut home = Rc::new(String::from("Random Vialla"));
        let rahim_home = Rc::clone(&home);
        let karim_home = Rc::clone(&home);
        println!("Rahim: {rahim_home}, Karim: {karim_home}")

    }
}
