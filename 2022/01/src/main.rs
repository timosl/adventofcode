extern crate core;

use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("The input file is missing!");

    let elf_calories: Vec<i32> = get_calories_of_each_elf(&input);
    let max_calories_of_single_elf = get_total_calories_of_best_elf(&elf_calories);
    let max_calories_of_top_three_elves = get_total_calories_of_top_three_elves(&elf_calories);
    println!("The top elf has {max_calories_of_single_elf} calories");
    println!("The top three elves have {max_calories_of_top_three_elves} calories");
}

fn get_calories_of_each_elf(input_string: &str) -> Vec<i32> {
    let mut elf_calories: Vec<i32> = Vec::new();
    let mut calories_of_current_elf = 0;
    for line in input_string.lines() {
        if line.is_empty() {
            elf_calories.push(calories_of_current_elf);
            calories_of_current_elf = 0;
        } else {
            let line_as_number = line.parse::<i32>().expect("Is {line} really a number?");
            calories_of_current_elf += line_as_number;
        }
    }
    elf_calories
}

fn get_total_calories_of_best_elf(calories_of_elves: &[i32]) -> i32 {
    let max_calories = calories_of_elves.iter().max();
    match max_calories {
        Some(calories) => *calories,
        None => 0
    }
}

fn get_total_calories_of_top_three_elves(calories_of_elves: &[i32]) -> i32 {
    let mut elf_calories = calories_of_elves.to_owned();
    elf_calories.sort();
    let mut max_calories_of_top_three_elves = 0;
    for (elves_inspected, sum_of_calories_for_elf) in elf_calories.iter().rev().enumerate() {
        if elves_inspected > 2 {
            break;
        }
        max_calories_of_top_three_elves += sum_of_calories_for_elf;
    }
    max_calories_of_top_three_elves
}
