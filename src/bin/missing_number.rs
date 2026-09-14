use std::io::{self, Read};

fn solve(n: i64, numbers: &[i64]) -> i64 {
    let sum: i64 = numbers.iter().sum();
    let expected_sum = n * (n + 1) / 2;
    expected_sum - sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: i64 = parts.next().unwrap().parse().unwrap();
    let numbers: Vec<i64> = parts.map(|s| s.parse().unwrap()).collect();
    println!("{}", solve(n, &numbers));
}
