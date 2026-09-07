fn solve(char: &str) -> i32 {
    if char.len() >= 1 {
        let mut current = 0;
        let mut max_len = 0;
        let mut prev = char.chars().next().unwrap();
        for c in char.chars().skip(1) {
            if c == prev {
                current += 1;
                max_len = max_len.max(current)
            } else {
                current = 1;
                prev = c
            }
        }
        max_len
    } else {
        0
    }
}

fn main() {
    println!("{:?}", solve(&"TEsskst"))
}
