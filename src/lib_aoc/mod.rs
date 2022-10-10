pub mod itertools;
pub mod primes;
pub mod unionfind;

use std::collections::{HashMap, HashSet};

pub fn parse_block_output(output: &HashSet<(u32, u32)>, width: u32) -> String {
    // Assume characters are always 4x6 in the output

    // Hand-drawn look-up table
    let mut character_codes = HashMap::with_capacity(6);
    character_codes.insert(16290430, 'A');
    character_codes.insert(4855902, 'C');
    character_codes.insert(283007, 'F');
    character_codes.insert(16531775, 'H');
    character_codes.insert(8263696, 'J');
    character_codes.insert(8757567, 'K');
    character_codes.insert(9329265, 'Z');

    let mut text = String::with_capacity((width / 5) as usize);
    let mut code: u32 = 0;
    let mut pixel_id: u32 = 1;
    for x in 0..width {
        let mut col_is_empty = true;
        for y in 0..6 {
            if output.contains(&(x, y)) {
                code |= pixel_id;
                col_is_empty = false;
            }
            pixel_id <<= 1;
        }
        if col_is_empty {
            // We should have a character now
            text.push(*character_codes.get(&code).unwrap_or(&'?'));
            // reset to look for next character
            code = 0;
            pixel_id = 1;
        }
    }
    text
}

pub fn print_block_output(height: u32, width: u32, dots: HashSet<(u32, u32)>) {
    println!();
    for y in 0..height {
        for x in 0..width {
            if dots.contains(&(x, y)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}
