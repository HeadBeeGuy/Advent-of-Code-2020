// Advent of Code 2020 Day 5
// I'm months late now, but I wanted to practice Rust a little more just for fun.
// I realized upon looking at the problem much later that this is just standard
// binary representation of integers. 
// It took me a while to figure out what part 2 meant, and I'm sure that there's
// faster ways to do it.

use std::fs;

fn code_to_integers(code: &str) -> (usize, usize) {
    if code.len() != 10 {
        return (0, 0);
    }

    let row: String = code[0..7]
        .chars()
        .into_iter()
        .take(7)
        .map(|char| match char {
            'F' => '0',
            'B' => '1',
            _ => 'X',
        })
        .collect();

    let column: String = code[7..10]
        .chars()
        .into_iter()
        .take(3)
        .map(|char| match char {
            'L' => '0',
            'R' => '1',
            _ => 'X',
        })
        .collect();

    // I found this very handy code at https://stackoverflow.com/a/27606979
    (
        usize::from_str_radix(&row, 2).unwrap(),
        usize::from_str_radix(&column, 2).unwrap(),
    )
}

fn seat_id(code: &str) -> usize {
    let (row, column) = code_to_integers(&code);
    (row * 8) + column
}

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't find the input file.");

    let mut max_seat_id: usize = 0;
    let mut seat_ids: Vec<usize> = vec![];

    for seat_code in input_file.lines() {
        let seat_id_score = seat_id(&seat_code);

        if seat_id_score > max_seat_id {
            max_seat_id = seat_id_score;
        }

        seat_ids.push(seat_id_score);
    }

    println!("The maximum Seat ID found was {}.", max_seat_id);

    seat_ids.sort();
    for current in seat_ids.windows(2) {
        if current[0] + 1 != current[1] {
            println!("Your Seat ID is {}.", current[0] + 1);
        }
    }
}
