extern crate stopwatch;

use stopwatch::{Stopwatch};

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn zero_sum(num_list: &mut Vec<i32>) -> Vec<(i32, i32, i32)> {
    let mut output = Vec::new();
    num_list.sort();
    let length = num_list.len();
    for i in 0..length-2 {
        let a = num_list[i];
        let mut start = i + 1;
        let mut end = length - 1;
        while start < end {
            let b = num_list[start];
            let c = num_list[end];
            if a + b + c == 0 {
                output.push((a, b, c));
                end -= 1;
            }
            else if a + b + c > 0 {
                end -= 1;
            }
            else {
                start += 1;
            }
        }
    }
    output.sort();
    output.dedup();
    return output
}


fn get_test_data(filename: &str) -> Vec<Vec<i32>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);
    let num_lists: Vec<Vec<i32>> = reader.lines()
        .filter_map(
            |l| l.ok().map(
                |s| s.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()))
        .collect();
    return num_lists
}

fn main() {
    let num_lists = get_test_data("../test_data.txt");
    for mut num_list in num_lists {
        let sw = Stopwatch::start_new();
        let output = zero_sum(&mut num_list);
        println!("Took {} ms.", sw.elapsed_ms());
        println!("{:?}", output);
    }
}
