use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

fn main() {
    const START: i32 = 50;
    const MAX: i32 = 99;

    let mut dial = START;
    let mut password = 0;

    if let Ok(rotations) = read_lines("input.txt") {
        for mut rotation in rotations.map_while(Result::ok) {
            let direction = rotation.remove(0);
            let count: i32 = rotation.parse().expect("NaN");

            dial = if direction == 'L' {
                dial - count
            } else {
                dial + count
            };

            dial %= 100;
            if dial < 0 {
                dial = MAX + 1 + dial;
            }

            if dial == 0 {
                password += 1;
            }
        }
    }

    println!("{}", password);
}

fn read_lines(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
