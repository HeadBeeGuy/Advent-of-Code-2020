// Advent of Code 2020 - Day 1
// I've seen an example of Rust code based on functional programming recently
// which looked really cool, and I have a suspicious that clever use of things
// like map or filter would make this much more compact and elegant.

// In the end, I just went with a really basic iteration strategy and tried to
// break out of it as soon as it found results. Of course, this will grow
// terribly as the list of numbers grows, but this is only Day 1, and Rust is
// fast enough to paper over my lack of sophistication!

use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't read the file!");
    let expenses: Vec<isize> = input_file
        .lines()
        .into_iter()
        .map(|i| i.parse().expect("This isn't an int!"))
        .collect();

    let mut found_two = false;
    let mut found_three = false;

    for a in 0..expenses.len() {
        for b in 1..expenses.len() {
            if expenses[a] + expenses[b] == 2020 && !found_two {
                println!(
                    "Positions {} and {} sum up to 2020: {} and {}.",
                    a, b, expenses[a], expenses[b]
                );
                println!("Their product is: {}.", expenses[a] * expenses[b]);
                found_two = true;
            }
            for c in 2..expenses.len() {
                if expenses[a] + expenses[b] + expenses[c] == 2020 {
                    println!(
                        "positions {}, {}, and {} sum up to 2020: {}, {}, and {}.",
                        a, b, c, expenses[a], expenses[b], expenses[c]
                    );
                    println!(
                        "Their product is: {}",
                        expenses[a] * expenses[b] * expenses[c]
                    );
                    found_three = true;
                    break;
                }
            }
            if found_two && found_three {
                break;
            }
        }
        if found_two && found_three {
            break;
        }
    }
}
