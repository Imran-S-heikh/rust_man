use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn run() {
    let num_list = vec![1, 2, 3, 4, 5, 6];

    let max_number = largest(&num_list);
    println!("{}", max_number);

    let pair = Pair::new(34,53);
    pair.cmp_display();

    // let another = Pair::new([2,3,4],[2,5,2]);
    // another.cmp_display();
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];

    for &item in list {
        if item > max {
            max = item;
        }
    }

    max
}
