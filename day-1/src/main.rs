use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_sum_for_line(line: String) -> u32 {
    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut first_visited = false;

    for c in line.chars() {
        if c.is_ascii_digit() {
            let result = c.to_digit(10);
            let value = match result {
                Some(value) => value,
                None => panic!("WTF!? {:#?}", result),
            };
            if !first_visited {
                first_digit = value;
                first_visited = true;
            }
            last_digit = value;
        }
    }
    // The digits are combined into a single number
    (first_digit * 10) + last_digit
}

fn main() {
    if let Ok(lines) = read_lines("/Users/gonchi/code/advent-of-code-2023/day-1/input.txt") {
        let mut total_sum = 0;
        for line in lines.flatten() {
            // We accumulate the sum
            total_sum += get_sum_for_line(line);
        }
        println!("The total sum is {}", total_sum);
    }
}
