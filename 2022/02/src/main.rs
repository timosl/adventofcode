extern crate core;

use std::fs::read_to_string;

const MOVE_VALUE_ROCK: i32 = 1;
const MOVE_VALUE_PAPER: i32 = 2;
const MOVE_VALUE_SCISSORS: i32 = 3;

const RESULT_VALUE_LOSE: i32 = 0;
const RESULT_VALUE_DRAW: i32 = 3;
const RESULT_VALUE_WIN: i32 = 6;

fn main() {
    let input = read_to_string("input.txt").expect("The input file is missing!");
    let rounds = get_game_rounds_from_file(&input);
    let results = rounds.iter()
        .map(determine_result_for_round)
        .collect::<Vec<i32>>();
    let total_score: i32 = results.iter().sum();
    println!("The total regular score is: {total_score}");

    let perfect_rounds = get_perfect_game_rounds_from_file(&input);
    let perfect_results = perfect_rounds.iter()
        .map(convert_perfect_round_to_normal_round)
        .map(|x| determine_result_for_round(&x))
        .collect::<Vec<i32>>();
    let total_perfect_score: i32 = perfect_results.iter().sum();
    println!("The total perfect score is: {total_perfect_score}");
}

#[derive(Clone, Copy)]
struct GameRound {
    enemy_move: EnemyMove,
    ally_move: AllyMove
}

#[derive(Clone, Copy)]
enum EnemyMove {
    Rock,
    Paper,
    Scissors
}

#[derive(Clone, Copy)]
enum AllyMove {
    Rock,
    Paper,
    Scissors
}

#[derive(Clone, Copy)]
enum RequiredOutcome {
    Lose,
    Draw,
    Win
}

#[derive(Clone, Copy)]
struct PerfectGameRound {
    enemy_move: EnemyMove,
    required_outcome: RequiredOutcome
}

fn get_game_rounds_from_file(file: &str) -> Vec<GameRound> {
    let mut results: Vec<GameRound> = Vec::new();
    for line in file.lines() {
        let enemy = line.chars().next().expect("The input file appears to be broken");
        let ally = line.chars().nth(2).expect("The input file appears to be broken");

        let enemy_move = match enemy {
            'A' => EnemyMove::Rock,
            'B' => EnemyMove::Paper,
            'C' => EnemyMove::Scissors,
            _ => panic!("The input file appears to be broken")
        };
        let ally_move = match ally {
            'X' => AllyMove::Rock,
            'Y' => AllyMove::Paper,
            'Z' => AllyMove::Scissors,
            _ => panic!("The input file appears to be broken")
        };
        results.push(GameRound {
            enemy_move,
            ally_move
        })
    }
    results
}

fn determine_result_for_round(round: &GameRound) -> i32 {
    match round.enemy_move {
        EnemyMove::Rock => match round.ally_move {
            AllyMove::Rock => MOVE_VALUE_ROCK + RESULT_VALUE_DRAW,
            AllyMove::Paper => MOVE_VALUE_PAPER + RESULT_VALUE_WIN,
            AllyMove::Scissors => MOVE_VALUE_SCISSORS + RESULT_VALUE_LOSE
        },
        EnemyMove::Paper => match round.ally_move {
            AllyMove::Rock => MOVE_VALUE_ROCK + RESULT_VALUE_LOSE,
            AllyMove::Paper => MOVE_VALUE_PAPER + RESULT_VALUE_DRAW,
            AllyMove::Scissors => MOVE_VALUE_SCISSORS + RESULT_VALUE_WIN
        },
        EnemyMove::Scissors => match round.ally_move {
            AllyMove::Rock => MOVE_VALUE_ROCK + RESULT_VALUE_WIN,
            AllyMove::Paper => MOVE_VALUE_PAPER + RESULT_VALUE_LOSE,
            AllyMove::Scissors => MOVE_VALUE_SCISSORS + RESULT_VALUE_DRAW
        }
    }
}

fn get_perfect_game_rounds_from_file(file: &str) -> Vec<PerfectGameRound> {
    let mut results: Vec<PerfectGameRound> = Vec::new();
    for line in file.lines() {
        let enemy = line.chars().next().expect("The input file appears to be broken");
        let outcome = line.chars().nth(2).expect("The input file appears to be broken");

        let enemy_move = match enemy {
            'A' => EnemyMove::Rock,
            'B' => EnemyMove::Paper,
            'C' => EnemyMove::Scissors,
            _ => panic!("The input file appears to be broken")
        };
        let required_outcome = match outcome {
            'X' => RequiredOutcome::Lose,
            'Y' => RequiredOutcome::Draw,
            'Z' => RequiredOutcome::Win,
            _ => panic!("The input file appears to be broken")
        };
        results.push(PerfectGameRound {
            enemy_move,
            required_outcome
        })
    }
    results
}

fn determine_ally_move_for_outcome(round: &PerfectGameRound) -> AllyMove {
    match round.enemy_move {
        EnemyMove::Rock => match round.required_outcome {
            RequiredOutcome::Lose => AllyMove::Scissors,
            RequiredOutcome::Draw => AllyMove::Rock,
            RequiredOutcome::Win => AllyMove::Paper
        },
        EnemyMove::Paper => match round.required_outcome {
            RequiredOutcome::Lose => AllyMove::Rock,
            RequiredOutcome::Draw => AllyMove::Paper,
            RequiredOutcome::Win => AllyMove::Scissors
        },
        EnemyMove::Scissors => match round.required_outcome {
            RequiredOutcome::Lose => AllyMove::Paper,
            RequiredOutcome::Draw => AllyMove::Scissors,
            RequiredOutcome::Win => AllyMove::Rock
        }
    }
}

fn convert_perfect_round_to_normal_round(round: &PerfectGameRound) -> GameRound {
    GameRound {
        enemy_move: round.enemy_move,
        ally_move: determine_ally_move_for_outcome(round)
    }
}
