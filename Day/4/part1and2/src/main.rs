mod puzzle_input;

use failure::Error;
use failure::err_msg;
use regex::Regex;
use regex::RegexSet;
use chrono::prelude::*;

use std::collections::BTreeMap;
use std::collections::HashMap;

// @returns the key of the entry with the highest value
fn hashmap_get_highest_value_entry(hash_map: &HashMap<u32, u32>) -> u32 {
    let mut highest_val: u32 = 0;
    let mut coresponding_key: u32 = 999999;

    for (key, val) in hash_map {
        if *val > highest_val {
            highest_val = *val;
            coresponding_key = *key;
        }
    }

    coresponding_key
}

fn array_get_index_of_highest_value(array: &[u32; 60]) -> usize {
    let mut highest_val: u32 = 0;
    let mut coresponding_index: usize = 999999;

    for index in 0..60 {
        if array[index] > highest_val {
            highest_val = array[index];
            coresponding_index = index;
        }
    }

    coresponding_index
}

fn main() -> Result<(), Error> {
    println!("Hello World!");

    let re = Regex::new(r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\] (?P<msg>.*)$")
        .expect("Regex did not compile correctly!");

    let mut entries: BTreeMap<DateTime<Utc>, String> = BTreeMap::new();
    
    for line in puzzle_input::LOG_ENTRIES.lines() {
        let caps = re.captures(line)
            .expect("Log line did not parse correctly!");
        
        let year: i32 = caps["year"].parse().unwrap();
        let month: u32 = caps["month"].parse().unwrap();
        let day: u32 = caps["day"].parse().unwrap();
        let hour: u32 = caps["hour"].parse().unwrap();
        let minute: u32 = caps["minute"].parse().unwrap();
        let msg: String = caps["msg"].to_string();
        let date_time: DateTime<Utc> = 
            Utc.ymd(year, month, day)
            .and_hms(hour, minute, 0);

        entries.insert(date_time, msg);
    }

    let options_regex = RegexSet::new(&[
        r"falls asleep",
        r"wakes up",
        r"Guard \#(?P<id>[0-9]+) begins shift",
    ]).unwrap();
    let guard_change_regex = 
        Regex::new(r"Guard \#(?P<id>[0-9]+) begins shift$")
        .unwrap();

    let mut sleep_counter: HashMap<u32, u32> = HashMap::new();
    let mut sleep_start_time: DateTime<Utc> = Utc::now();
    let mut current_guard_id: u32 = 0;
    for (date, msg) in &entries {
        let matches = options_regex.matches(&msg);
        if matches.matched(0) { //falls asleep
            sleep_start_time = *date;
        }else if matches.matched(1) { //wakes up
            let delta: u32 = date.signed_duration_since(sleep_start_time).num_minutes() as u32;
            if !sleep_counter.contains_key(&current_guard_id) {
                sleep_counter.insert(current_guard_id, 0);
            }
            *sleep_counter.get_mut(&current_guard_id).unwrap() += delta;
        }else if matches.matched(2) { //switch guard
            let caps_id = guard_change_regex.captures(&msg).unwrap();
            let id: u32 = caps_id["id"].parse().unwrap();
            current_guard_id = id;
        }
    }

    println!("Guard sleep times: \n{:?}", &sleep_counter);

    let sleepiest_guard = hashmap_get_highest_value_entry(&sleep_counter);

    //Find the best minute
    let mut sleepiest_minute: [u32; 60] = [0; 60];
    for (date, msg) in &entries {
        let matches = options_regex.matches(&msg);
        if matches.matched(0) { //falls asleep
            sleep_start_time = *date;
        }else if matches.matched(1) { //wakes up
            if current_guard_id == sleepiest_guard {
                let delta: u32 = date.signed_duration_since(sleep_start_time).num_minutes() as u32;
                for min in sleep_start_time.minute()..sleep_start_time.minute()+delta {
                    sleepiest_minute[(min % 60) as usize] += 1;
                }
            }
        }else if matches.matched(2) { //switch guard
            let caps_id = guard_change_regex.captures(&msg).unwrap();
            let id: u32 = caps_id["id"].parse().unwrap();
            current_guard_id = id;
        }
    }

    println!("Guard {} minute sleeping chart: {:?}", sleepiest_guard, sleepiest_minute.to_vec());
    let best_minute = array_get_index_of_highest_value(&sleepiest_minute);

    println!("\n\nSleepiest guard is: {}. With {} minutes! The most sleepiest minute was {}.\nSo the solution for part1 is: {}", 
        sleepiest_guard,
        sleep_counter[&sleepiest_guard],
        best_minute,
        sleepiest_guard * (best_minute as u32));

    let mut sleepmap: HashMap<u32, [u32; 60]> = HashMap::new();
    for (date, msg) in &entries {
        let matches = options_regex.matches(&msg);
        if matches.matched(0) { //falls asleep
            sleep_start_time = *date;
        }else if matches.matched(1) { //wakes up
            let delta: u32 = date.signed_duration_since(sleep_start_time).num_minutes() as u32;
            if !sleepmap.contains_key(&current_guard_id) {
                sleepmap.insert(current_guard_id, [0; 60]);
            }
            for min in sleep_start_time.minute()..sleep_start_time.minute()+delta {
                sleepmap.get_mut(&current_guard_id).unwrap()[(min % 60) as usize] += 1;
            }
            // if !sleep_counter.contains_key(&current_guard_id) {
            //     sleep_counter.insert(current_guard_id, 0);
            // }
            // *sleep_counter.get_mut(&current_guard_id).unwrap() += delta;
        }else if matches.matched(2) { //switch guard
            let caps_id = guard_change_regex.captures(&msg).unwrap();
            let id: u32 = caps_id["id"].parse().unwrap();
            current_guard_id = id;
        }
    }

    let mut best_guard_id = 999999999;
    let mut best_sleep_minute = 0;
    let mut best_sleep_amount_minutes = 0;
    for (guard_id, array) in &sleepmap {
        for index in 0..60 {
            if array[index] > best_sleep_amount_minutes {
                best_guard_id = *guard_id;
                best_sleep_amount_minutes = array[index];
                best_sleep_minute = index as u32;
            }
        }
    }

    println!("Best guard for part 2: {} with minute: {}, (which has been counted {} times, same number: {}) So the awnser is: {}", 
    best_guard_id, 
    best_sleep_minute, 
    sleepmap[&best_guard_id][best_sleep_minute as usize],
    best_sleep_amount_minutes,
    best_guard_id * best_sleep_minute);

    Ok(())
}
