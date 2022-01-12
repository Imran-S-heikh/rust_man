#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u8,
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(self) -> u32 {
        self.height * self.width
    }
}

pub fn run() {
    let imran = User {
        name: String::from("Imran Shaikh"),
        email: String::from("imran@gmail.com"),
        age: 22,
    };

    let rect1 = Rectangle{
        height: 30,
        width: 40
    };

    println!("{:#?}",rect1);
    println!("The area of the Rectangle is: {}",rect1.area());

    println!("{:#?}", imran)
}
