extern crate support;

use std::cmp;
use std::u32;

fn checksum2(string: String) -> u32 {
    let mut total = 0;
    for line in string.split("\n") {
        let mut sorted_line = line.split_whitespace()
            .map(|c| c.to_string().parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        sorted_line.sort_by(|a, b| b.cmp(a));

        'mainloop: for i in 0..sorted_line.len() {
            for j in i + 1..sorted_line.len() {
                if sorted_line[i] % sorted_line[j] == 0 {
                    total += sorted_line[i] / sorted_line[j];
                    break 'mainloop;
                }
            }
        }
    }
    total
}

fn checksum(string: String) -> u32 {
    let mut total = 0;
    for line in string.split("\n") {
        let nums = line.split_whitespace()
            .map(|c| c.to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mx = nums.iter().fold(0, |agg, next| cmp::max(agg, *next));
        let mn = nums.iter().fold(u32::MAX, |agg, next| cmp::min(agg, *next));
        total += mx - mn;
    }
    total
}

fn main() {
    let captcha = support::read_input("2-1").unwrap().trim().to_owned();
    println!("Part 1: {}", checksum(captcha.clone()));
    println!("Part 2: {}", checksum2(captcha.clone()));
}
