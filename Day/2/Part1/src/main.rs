
mod puzzle_input;

use failure::Error;

fn char_to_idx(ch: char) -> usize{
    const OFFSET: u8 = 'a' as u8;
    let ch_u8 = ch as u8;
    return (ch_u8 - OFFSET) as usize;
}

fn check_ids(char_count: &[u8; 28]) -> (bool, bool) {
    let mut got_pair = false;
    let mut got_three = false;

    for i in 0..28 {
        got_pair = got_pair || char_count[i] == 2;
        got_three = got_three || char_count[i] == 3;
    }

    return (got_pair, got_three);
}

#[allow(dead_code)]
fn print_char_count(char_count: &[u8; 28]) {
    for count in char_count.iter() {
        print!(" {}", count);
    }
    println!("");
}

fn main() -> Result<(), Error>{
    assert!(puzzle_input::NUM_BOX_IDS == puzzle_input::BOX_IDS.lines().count());

    let mut char_count: [u8; 28] = [0; 28];
    let mut num_pairs: u32 = 0;
    let mut num_threes: u32 = 0;

    for line in puzzle_input::BOX_IDS.lines() {
        let chars = line.chars();
        for ch in chars {
            let char_idx = char_to_idx(ch);
            char_count[char_idx] += 1;
        }

        let (pair_available, three_available) = check_ids(&char_count);
        num_pairs += pair_available as u32;
        num_threes += three_available as u32;

        char_count = [0; 28];
    }
    
    println!("Total pairs: {}. Total threes: {}. Checksum: {}", 
        num_pairs, num_threes, num_pairs * num_threes);
    Ok(())
}
