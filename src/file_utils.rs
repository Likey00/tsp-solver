use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(file_name: &str) -> Vec<Vec<f64>> {
    let f = BufReader::new(
        File::open(file_name).unwrap()
    );

    f.lines()
        .map(|line| line_to_floats(line.unwrap()))
        .collect()
}

fn line_to_floats(line: String) -> Vec<f64> {
    line.split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .map(|num| if num == 0. { f64::INFINITY } else { num })
        .collect()
}