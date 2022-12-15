use std::fs::read_to_string;

fn main() {
    let input_data = read_input_file();
    let rucksacks = split_input_into_rucksacks(&input_data);
    let result_priority = rucksacks
        .into_iter()
        .map(split_items_into_compartments)
        .map(find_item_in_both_compartments)
        .map(determine_priority_for_item)
        .sum::<u32>();

    println!("The priority of duplicate items in each rucksack is {result_priority}");

    let team_priority = split_into_teams(&input_data)
        .into_iter()
        .map(find_item_in_team_rucksacks)
        .map(determine_priority_for_item)
        .sum::<u32>();

    println!("The priority of all teams is {team_priority}");
}

fn find_item_in_team_rucksacks(team: Vec<String>) -> char {
    for first_char in team.get(0).unwrap().chars() {
        for second_char in team.get(1).unwrap().chars() {
            for third_char in team.get(2).unwrap().chars() {
                if first_char == second_char && second_char == third_char {
                    return first_char
                }
            }
        }
    }
    'a'
}

fn split_into_teams(input_data: &str) -> Vec<Vec<String>> {
    let mut rucksacks = split_input_into_rucksacks(input_data);
    let mut teams: Vec<Vec<String>> = Vec::new();
    while !rucksacks.is_empty() {
        let (new_team, remaining) = rucksacks.split_at(3);
        teams.push(new_team.to_vec());
        rucksacks = remaining.to_vec();
    }
    teams
}

fn determine_priority_for_item(item: char) -> u32 {
    let code_point_value = (item as u32) - ('A' as u32);
    (if code_point_value < 26 {
        code_point_value + 26
    } else {
        (item as u32) - ('a' as u32)
    } + 1)
}

fn find_item_in_both_compartments(compartments: (String, String)) -> char {
    for first_char in compartments.0.chars() {
        for second_char in compartments.1.chars() {
            if first_char == second_char {
                return first_char
            }
        }
    }
    '0'
}

fn split_items_into_compartments(rucksack: String) -> (String, String) {
    let rucksack_size = rucksack.len();
    let split_point = rucksack_size / 2;
    let (first_compartment, second_compartment) = rucksack.split_at(split_point);
    (first_compartment.to_string(), second_compartment.to_string())
}

fn split_input_into_rucksacks(input_data: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in input_data.lines() {
        result.push(line.to_string());
    }
    result
}

fn read_input_file() -> String {
    read_to_string("input.txt").expect("The input file is missing")
}
