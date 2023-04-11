use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(file_name: &str) -> Vec<Vec<i32>> {
    let f = BufReader::new(
        File::open(file_name).unwrap()
    );

    f.lines()
        .map(|line| line_to_ints(line.unwrap()))
        .collect()
}

fn line_to_ints(line: String) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .map(|num| if num == 0 { i32::MAX } else { num })
        .collect()
}