mod puzzle_input;

fn react_polymer(chars: &str) -> String {
    let char_amount = chars.as_bytes().len();
    let mut result = chars.as_bytes().to_vec();
    //Why is it char_amount-2????
    for index in (1..(char_amount-2)).rev() {
        let ch = result[index];
        let previous = result[index-1];
        if ch != previous && (ch == previous.to_ascii_uppercase() || ch.to_ascii_uppercase() == previous) {
            result.remove(index);
            result.remove(index - 1usize);
        }
    }

    String::from_utf8(result.clone()).unwrap()
}

fn main() {
    // println!("Hello, world!");

    //println!("Lol hello: {}", puzzle_input::TEXT_UNITS);
    
    let mut best_string = String::from("NO BEST STRING FOUND");
    let mut shortest_string_len = usize::max_value();
    for u8_to_remove in ('a' as u8)..('z' as u8) {
        let char_to_remove = u8_to_remove as char;
        let modified: String = puzzle_input::TEXT_UNITS.chars().filter(|&c| c.to_ascii_lowercase() != char_to_remove ).collect();
        let reacted = react_polymer(&modified);
        if reacted.len() < shortest_string_len {
            shortest_string_len = reacted.len();
            best_string = reacted;
        }
    }
    //let result = react_polymer(puzzle_input::TEXT_UNITS);

    println!("\n\nNow the resulting string: {}", &best_string);
    println!("With a size of: {}", shortest_string_len);
}
