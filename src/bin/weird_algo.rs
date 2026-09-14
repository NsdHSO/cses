use std::io;

fn solve(mut n: i32) {
    print!("{}", n);
    while n != 1 {
        if (n % 2 == 0) {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
        print!(" {}", n)
    }
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    solve(n);
}
