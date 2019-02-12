mod puzzle_input;

fn main() {
    println!("Hello, world!");

    println!("Lol hello: {}", puzzle_input::TEXT_UNITS);

    let char_amount = puzzle_input::TEXT_UNITS.as_bytes().len();
    let mut result = puzzle_input::TEXT_UNITS.as_bytes().to_vec();
    for index in (1..char_amount).rev() {
        let ch = result[index];
        let previous = result[index-1];
        if ch != previous && (ch == previous.to_ascii_uppercase() || ch.to_ascii_uppercase() == previous) {
            result.remove(index);
            result.remove(index - 1usize);
        }
    }

    println!("\n\nNow the resulting string: {:?}", String::from_utf8(result.clone()).unwrap());
    println!("With a size of: {}", result.len());
}
