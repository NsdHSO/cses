use std::io::{self, Read};

fn solve(s: &str) -> i32 {
    let mut max_len = 0;
    let mut current = 0;
    let mut prev: Option<char> = None;

    for c in s.chars() {
        if Some(c) == prev {
            current += 1;
        } else {
            current = 1;
            prev = Some(c);
        }
        max_len = max_len.max(current);
    }
    max_len
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("{}", solve(input));
}
