mod command_data;

use failure::Error;
use failure::format_err;

use std::collections::HashSet;

const MAX_LAPS: usize = 150;

fn parse_instructions(instruction_arr: &mut [i32; command_data::NUM_COMMANDS], unparsed_commands: &str){
    let mut current_instruction: usize = 0;

    let lines_it = unparsed_commands.lines();    
    for line in lines_it {
        let frequency_change: i32 = line.parse()
            .expect("Cannot parse change of frequency into number!");

        instruction_arr[current_instruction] = frequency_change;
        current_instruction += 1;
    }
}

fn main() -> Result<(), Error> {
    assert!(command_data::NUM_COMMANDS == command_data::COMMANDS.lines().count());
    let mut instructions: [i32; command_data::NUM_COMMANDS] = [0; command_data::NUM_COMMANDS];

    parse_instructions(&mut instructions, &command_data::COMMANDS);

    let mut current_frequency = 0;
    let mut total_laps = 0;

    let mut previous_frequencies: HashSet<i32> = HashSet::with_capacity(MAX_LAPS * command_data::NUM_COMMANDS);
    for lap in 0..MAX_LAPS {
        for instruction in instructions.iter() {
            previous_frequencies.insert(current_frequency);

            current_frequency += instruction;
            
            if(previous_frequencies.contains(&current_frequency)){
                println!("SUCCES: Found duplicate frequency! #{}. After {} laps.", current_frequency, total_laps);
            
                return Ok(());
            }
        }

        println!("Ran the list {} number of times thus far. current frequency: {}", lap, current_frequency);
        total_laps += 1;
    }

    return Err(format_err!("ERROR: Could not find duplicate frequency, \
        after {} tries!", total_laps));
}
