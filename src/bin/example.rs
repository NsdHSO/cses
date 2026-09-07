use std::io::{self, BufRead, BufReader, Write};

fn solve<R: BufRead, W: Write>(input: &mut R, output: &mut W) {
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut sum: i64 = 0;
    for _ in 0..n {
        let x: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        sum += x;
    }
    writeln!(output, "{}", sum).unwrap();
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = io::BufWriter::new(stdout.lock());
    solve(&mut reader, &mut writer);
}