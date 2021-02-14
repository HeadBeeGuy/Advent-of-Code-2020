use std::fs;

fn main() {
    let input_file = fs::read_to_string("input_copypaste.txt").expect("Couldn't read the file!");
    
    let entries: Vec<&str> = input_file.split("\r\n\r\n").collect();

    let mut valid_matches: usize = 0;
    for passport in entries {
        let has_cid: bool = passport.matches("cid").count() == 1;
        let colon_count: usize = passport.matches(":").count();
        if (colon_count == 7 && !has_cid ) || (colon_count == 8 && has_cid) {
            valid_matches += 1;
        }
    }

    println!("Found {} valid passports.", valid_matches);
}
