use std::fs::read_to_string;

struct ElfPair {
    first: AssignedSection,
    second: AssignedSection
}

struct AssignedSection {
    lower_bound: u32,
    upper_bound: u32
}

fn main() {
    let sections_fully_covering_another = read_input()
        .into_iter()
        .filter(is_one_section_covering_the_other)
        .count();

    println!("Amount of pairs where one section fully covers another: {sections_fully_covering_another}");

    let sections_overlapping_at_all = read_input()
        .into_iter()
        .filter(is_section_pair_overlapping)
        .count();

    println!("Amount of pairs overlapping at all: {sections_overlapping_at_all}")
}

fn is_section_pair_overlapping(input: &ElfPair) -> bool {
    !(input.first.upper_bound < input.second.lower_bound || input.second.upper_bound < input.first.lower_bound)
}

fn is_one_section_covering_the_other(input: &ElfPair) -> bool {
    (input.first.lower_bound <= input.second.lower_bound && input.first.upper_bound >= input.second.upper_bound)
        || (input.second.lower_bound <= input.first.lower_bound && input.second.upper_bound >= input.first.upper_bound)
}

fn read_input() -> Vec<ElfPair> {
    return read_to_string("input.txt")
        .unwrap()
        .lines()
        .into_iter()
        .map(convert_to_assigned_section)
        .collect::<Vec<ElfPair>>();
}

fn convert_to_assigned_section(line: &str) -> ElfPair {
    let a = line.split_once(',').unwrap();
    let first_elf_values = a.0.split_once('-').unwrap();
    let second_elf_values = a.1.split_once('-').unwrap();
    ElfPair {
        first: AssignedSection {
            lower_bound: first_elf_values.0.parse::<u32>().unwrap(),
            upper_bound: first_elf_values.1.parse::<u32>().unwrap()
        },
        second: AssignedSection {
            lower_bound: second_elf_values.0.parse::<u32>().unwrap(),
            upper_bound: second_elf_values.1.parse::<u32>().unwrap()
        },
    }
}
