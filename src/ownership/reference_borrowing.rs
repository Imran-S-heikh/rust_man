pub fn ref_borrow() {
    let reference_to_nothing = dangle();

    println!("{}", reference_to_nothing);
}

fn dangle() -> String {
    let s = String::from("Hello World");
    s
}
