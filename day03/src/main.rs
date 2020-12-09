// Advent of Code 2020 Day 3
// As always, I think my solution is unsophisticated, but Rust makes it faster
// than it deserves. I just turn the tree file into a big HashMap that has
// values wherever there's a tree at that coordinate. Then I iterate a path
// according to the given slope and check how many times the present coordinate
// matches a tree location. Fortunately I can just use modulo to simulate a
// repeating horizontal field.
// I thought Part 2 would ask to find the optimal slope out of all possible
// slopes. Fortunately it was more merciful than that!

use std::collections::HashMap;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't read the file!");
    let tree_file = input_file.lines().enumerate();

    let mut tree_map: HashMap<(usize, usize), bool> = HashMap::new();

    for y in tree_file {
        for x in y.1.chars().enumerate() {
            match x.1 {
                '#' => {
                    tree_map.insert((x.0, y.0), true);
                }
                _ => {}
            }
        }
    }
    let vertical_size = input_file.lines().count();
    let horizontal_size: usize = input_file.lines().next().unwrap().len();

    let mut tree_product: usize = 1;

    for path in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        // not stricly necessary, but I find it useful to rename these
        let x_step: usize = path.0;
        let y_step: usize = path.1;

        let mut trees_whacked: usize = 0;
        let mut current_x: usize = 0;
        for current_y in (0..vertical_size).step_by(y_step) {
            if tree_map.contains_key(&(current_x, current_y)) {
                trees_whacked += 1;
            }
            current_x = (current_x + x_step) % horizontal_size;
        }

        println!(
            "By going {} units over and {} down, you ran into {} trees.",
            x_step, y_step, trees_whacked
        );
        tree_product *= trees_whacked;
    }

    println!(
        "Multiplying the trees from all of these together, we get {}.",
        tree_product
    );
}
