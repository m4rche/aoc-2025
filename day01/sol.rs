use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

const START: i32 = 50;
const MAX: i32 = 99;

fn main() {
    let mut dial = START;

    let mut password_0 = 0;
    let mut password_1 = 0;

    if let Ok(rotations) = read_lines("input.txt") {
        for mut rotation in rotations.map_while(Result::ok) {
            let direction = rotation.remove(0);
            let count: i32 = rotation.parse().expect("NaN");

            for _ in 0..count {
                if direction == 'L' {
                    dial -= 1;
                } else {
                    dial += 1;
                }
                if dial > MAX {
                    dial = 0;
                } else if dial < 0 {
                    dial = MAX;
                }

                if dial == 0 {
                    password_1 += 1;
                }
            }

            if dial == 0 {
                password_0 += 1;
            }
        }
        println!("{} {}", password_0, password_1);
    }
}

fn read_lines(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
