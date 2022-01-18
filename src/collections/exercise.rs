use std::{collections::HashMap, cmp::Ordering};

pub fn find_median_mode() {

    // Given Vector
    let mut array = vec![1, 3, 5, 2, 6, 4, 7,7];
    array.sort();



    // Find The Median
    let length = array.len();
    let median: f32;

    if length % 2 == 1 {
        median = array[length / 2] as f32;
    } else {
        let first = array[length / 2] as f32;
        let second = array[length / 2 - 1] as f32;

        median = (first + second) / 2.0;
    }


    // Find The Mode
    let mut mode: Vec<i32> = vec![];
    let mut current_count = 1;

    let mut count_hash: HashMap<i32, i8> = HashMap::new();

    for &item in &array {
        let count = count_hash.entry(item).or_insert(0);
        *count += 1;
    }

    for (key, value) in &count_hash {

        if value <= &1{continue;} 

        match value.cmp(&current_count) {
            Ordering::Less => continue,
            Ordering::Equal => {
                mode.push(*key);
            },
            Ordering::Greater => {
                mode = vec![*key];
                current_count = *value;
            }
        }
    }

    println!("Median: {:?}", median);
    println!("Mode: {:?}", mode);
}
