#[derive(PartialEq, Debug)]
struct Shoe {
    size: u8,
    name: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u8) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

pub fn iterator() {
    let array = vec![3, 2, 5];

    let array_iter: Vec<i32> = array.iter().map(|x| x * x).collect();

    for item in array_iter {
        println!("{item}");
    }

    let shoes = vec![
        Shoe {
            size: 41,
            name: String::from("Sneaker"),
        },
        Shoe {
            size: 42,
            name: String::from("Sandal"),
        },
        Shoe {
            size: 23,
            name: String::from("Boot"),
        },
    ];

    let size_41 = shoes_in_size(shoes, 41);

    assert_eq!(
        size_41,
        vec![Shoe {
            size: 41,
            name: String::from("Sneaker")
        }]
    )
}
