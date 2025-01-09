use std::collections::HashMap;

fn calc_median(data: &Vec<i32>) -> i32 {
    let length = data.len();

    let idx;

    if length % 2 == 0 {
        idx = length / 2;
    } else {
        idx = ((length / 2) + ((length / 2) + 1)) / 2;
    }

    data[idx]
}

fn calc_mode(data: &Vec<i32>) -> i32 {
    let mut tracker = HashMap::new();

    for v in data {
        let count = tracker.entry(*v).or_insert(0);
        *count += 1;
    }

    let mut max_value = -1;
    let mut max_count = 0;
    for (value, count) in tracker {
        if count > max_count {
            max_value = value;
            max_count = count;
        }
    }

    max_value
}

fn main() {
    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let mut input = vec![
        4, 5, 6, 1, 1, 1, 3, 1, 1, 5, 9, 99, 1231, 5654, 1, 5, 77, 3, 99, 99, 99, 99,
    ];

    input.sort();

    let median = calc_median(&input);
    println!("median: {}", median);

    let mode = calc_mode(&input);
    println!("mode: {}", mode);
}
