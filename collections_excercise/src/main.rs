use std::collections::HashMap;

fn main() {
    let mode = get_mode(&vec![1, 2, 3, 2, 3, 1, 1, 1, 2, 3, 1, 2, 2, 2, 2]);
    println!("{mode}");

    let median = get_median(&vec!{5, 2, 3, 1, 4});
    println!("{median}");
}

fn get_median(vec: &Vec<i32>) -> i32 {
    let mut sorted_vec = vec.clone();
    sorted_vec.sort();

    sorted_vec[vec.len() / 2]
}

fn get_mode(vec: &Vec<i32>) -> i32 {
    let mut histogram = HashMap::new();
    for i in vec {
        let count =  histogram.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut max = (0, 0);
    for (value, count) in histogram {
        if count > max.1 {
            max = (value, count);
        }
    }

    max.0
}
