fn add_str(str_ref: &mut String, val: &str) {
    str_ref.push_str(val)
}

fn name_format(first_name: &String,last_name: &String)-> String {
    format!("{} {}",first_name,last_name)
}

pub fn run() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let mut my_string = String::from("Hello World, My name is Imran Sheikh");

    add_str(&mut my_string, "Later added text");
    my_string.push_str(" Man Are You There");

    println!("{}", my_string);

    println!("{:?}", v);

    let hello = String::from("Hello");
    let world = String::from("World!!");

    let hello_world = format!("{} {}",hello,world);
    let l_hello_world = hello + " " + &world;
    // let a_hello_world = hello + &world;

    println!("{}",hello_world);
    println!("{}",l_hello_world);

    let first_name = String::from("Imran");
    let last_name = String::from("Sheikh");

    let name = name_format(&first_name, &last_name);

    println!("{} {} {}",name,first_name,last_name)
}
