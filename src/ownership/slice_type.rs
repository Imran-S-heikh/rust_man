pub fn run() {
    let s = String::from("Imran Shaikh");

    let my_string = "Hello World";
    println!("{}",first_word(&my_string[..]));

    println!("{}", first_word(&s))
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {

        println!("Index {}, Item {}, {}", i, item,b' ');

        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
