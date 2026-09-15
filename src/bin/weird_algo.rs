use std::io;

fn solve(mut n: i64) {
    let mut out = String::new();
    out.push_str(&format!("{} ", n));
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        out.push_str(&format!("{} ", n));
    }
    print!("{}", out)
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();
    solve(n);
}
