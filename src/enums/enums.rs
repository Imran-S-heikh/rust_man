enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin)->u8 {
   match coin {
       Coin::Penny => 1,
       Coin::Nickel => 5,
       Coin::Dime => 10,
       Coin::Quarter => 25
   } 
}

pub fn run() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move {x: i32,y: i32},
        Write(String),
        ChangeColor(i32,i32,i32)
    }

    impl Message {
        fn hello(&self) {
            println!("Self is called {:?}",self)
        }
    }

    let m = Message::Write(String::from("Hello World!!"));
    m.hello();

    let money = value_in_cents(Coin::Dime);
    println!("This is the converted money: {}",money)


}