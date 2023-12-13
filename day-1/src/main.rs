use std::collections::{HashMap, HashSet};
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

fn is_a_number_name(partial_string: &String) -> bool {
    let number_names: HashSet<&str> = HashSet::from([
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]);

    number_names.contains(&partial_string as &str)
}

fn replace_number_names_for_numbers(line: &str) -> String {
    // Note that the largest string is len 5
    let number_names_to_number_mapping: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),   // Len 3
        ("two", "2"),   // Len 3
        ("three", "3"), // Len 5
        ("four", "4"),  // Len 4
        ("five", "5"),  // Len 4
        ("six", "6"),   // Len 3
        ("seven", "7"), // Len 5
        ("eight", "8"), // Len 5
        ("nine", "9"),  // Len 4
    ]);

    // We need to scan the whole string, trying to find the longest
    // substring that matches a number name
    let mut current_index = 0;
    let mut new_line: Vec<String> = Vec::new();
    // Let's keep track of changes to the new line
    let mut new_line_changed: bool;
    // We need an accumulator to keep the chars that we have seen so far
    let mut accumulator: Vec<char> = Vec::new();

    // Go char by char in the original line
    while current_index < line.len() {
        // First we check if the current index is an ASCII digit
        let current_char = line.chars().nth(current_index).unwrap();
        if current_char.is_ascii_digit() {
            // Here we just go ahead and push it, skip the complicated logic
            new_line.push(current_char.to_string());
            // Don't forget to move the index
            current_index += 1;
            continue;
        }

        // We need to clear the accumulator to avoid contamination between loops
        accumulator.clear();
        new_line_changed = false;
        // Make a copy of the string starting from the last char that was
        // a starting point
        let mut slice_end_index = current_index + 5;
        if slice_end_index > line.len() {
            slice_end_index = line.len();
        }
        let current_slice = &line[current_index..slice_end_index];
        if current_slice.is_empty() {
            break;
        }

        // And we start looking at the current slice
        for current_sub_char in current_slice.chars() {
            // Let's add to the accumulator
            accumulator.push(current_sub_char);
            // Let's get the string that we have so far
            let current_string: String = accumulator.clone().into_iter().collect();
            // And now we check if it's any of the known number names
            if is_a_number_name(&current_string) {
                if let Some(number_string) =
                    number_names_to_number_mapping.get(&current_string as &str)
                {
                    new_line.push(number_string.to_string());
                    new_line_changed = true;
                    // Skip this many slots in the line
                    current_index += accumulator.len();
                    break;
                }
            }
        }

        if !new_line_changed {
            // Here we gave up. First we need to add the first element in the accumulator
            // to the new_line to not lose it. Note that the rest of the accumulator
            // is discarded and that's OK! we need to keep exploring the rest of the string
            new_line.push(accumulator[0].to_string());
            // We move one forward
            current_index += 1;
        }
    }

    new_line.join("")
}

fn get_sum_for_line(line: &str) -> u32 {
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
            let processed_line = replace_number_names_for_numbers(&line);
            let sum_for_line = get_sum_for_line(&processed_line);
            println!(
                "Original: {} - Processed: {} - Sum: {}",
                line, processed_line, sum_for_line
            );
            // We accumulate the sum
            total_sum += sum_for_line;
        }
        println!("The total sum is {}", total_sum);
    }
}
