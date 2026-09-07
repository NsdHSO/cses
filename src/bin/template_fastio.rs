use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = io::BufWriter::new(stdout.lock());

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let x: i64 = line.trim().parse().unwrap();
        // process x
    }

    writeln!(writer, "answer").unwrap();
}