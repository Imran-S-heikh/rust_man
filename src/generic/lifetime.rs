fn largest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() < second.len() {
        second
    }else{
        first
    }
}

fn life<'a>(value: &'a str,another: &i32)->&'a str {
    value
}

pub fn lifetime() {

    let first = "imran sheikh";
    let second = String::from("inmdaran sheikhd sai");

    // let third = second;
    let max = largest(first,second.as_str());

    println!("{}",life(max,&34))
}
