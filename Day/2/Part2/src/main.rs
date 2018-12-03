mod puzzle_input;

use failure::Error;
use failure::err_msg;

fn main() -> Result<(), Error> {
    assert!(puzzle_input::NUM_BOX_IDS == puzzle_input::BOX_IDS.lines().count());

    'outer: for line in puzzle_input::BOX_IDS.lines() {
        'inner: for line2 in puzzle_input::BOX_IDS.lines() {
            let mut chars_it = line.chars();
            let mut chars_it2 = line2.chars();

            assert!(line.chars().count() == line2.chars().count());

            let mut difference_index: i32 = -1;

            for i in 0..line.chars().count() {
                let ch = chars_it.next();
                let ch2 = chars_it2.next();

                if ch != ch2 {
                    if difference_index != -1 {
                        continue 'inner;
                    } else {
                        difference_index = i as i32;
                    }
                }
            }

            if difference_index != -1 {
                println!("Found id match! Char index: {}. \nStrings: {} and {}\n", difference_index, line, line2);

                let mut line_copy = String::from(line);
                line_copy.remove(difference_index as usize);
                println!("Awnser: {}", line_copy);

                println!("");
                return Ok(());
            }
        }
    }

    Err(err_msg("Could not find a matching Id!"))
}
