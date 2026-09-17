use std::io::{self};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let array: Vec<i32> = parts.map(|p| p.parse().unwrap()).collect();

    print!("{:#?}", &array)
}
