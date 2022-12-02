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
        .map(|x| determine_result_for_round(x))
        .collect::<Vec<i32>>();
    let total_score: i32 = results.iter().sum();
    println!("The total regular score is: {total_score}");

    let perfect_rounds = get_perfect_game_rounds_from_file(&input);
    let perfect_results = perfect_rounds.iter()
        .map(|x| convert_perfect_round_to_normal_round(x))
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
    ROCK,PAPER,SCISSORS
}

#[derive(Clone, Copy)]
enum AllyMove {
    ROCK,PAPER,SCISSORS
}

#[derive(Clone, Copy)]
enum RequiredOutcome {
    LOSE,DRAW,WIN
}

#[derive(Clone, Copy)]
struct PerfectGameRound {
    enemy_move: EnemyMove,
    required_outcome: RequiredOutcome
}

fn get_game_rounds_from_file(file: &String) -> Vec<GameRound> {
    let mut results: Vec<GameRound> = Vec::new();
    for line in file.lines() {
        let enemy = line.chars().nth(0).expect("The input file appears to be broken");
        let ally = line.chars().nth(2).expect("The input file appears to be broken");

        let enemy_move = match enemy {
            'A' => EnemyMove::ROCK,
            'B' => EnemyMove::PAPER,
            'C' => EnemyMove::SCISSORS,
            _ => panic!("The input file appears to be broken")
        };
        let ally_move = match ally {
            'X' => AllyMove::ROCK,
            'Y' => AllyMove::PAPER,
            'Z' => AllyMove::SCISSORS,
            _ => panic!("The input file appears to be broken")
        };
        results.push(GameRound {
            enemy_move,
            ally_move
        })
    }
    return results;
}

fn determine_result_for_round(round: &GameRound) -> i32 {
    return match round.enemy_move {
        EnemyMove::ROCK => match round.ally_move {
            AllyMove::ROCK => MOVE_VALUE_ROCK + RESULT_VALUE_DRAW,
            AllyMove::PAPER => MOVE_VALUE_PAPER + RESULT_VALUE_WIN,
            AllyMove::SCISSORS => MOVE_VALUE_SCISSORS + RESULT_VALUE_LOSE
        },
        EnemyMove::PAPER => match round.ally_move {
            AllyMove::ROCK => MOVE_VALUE_ROCK + RESULT_VALUE_LOSE,
            AllyMove::PAPER => MOVE_VALUE_PAPER + RESULT_VALUE_DRAW,
            AllyMove::SCISSORS => MOVE_VALUE_SCISSORS + RESULT_VALUE_WIN
        },
        EnemyMove::SCISSORS => match round.ally_move {
            AllyMove::ROCK => MOVE_VALUE_ROCK + RESULT_VALUE_WIN,
            AllyMove::PAPER => MOVE_VALUE_PAPER + RESULT_VALUE_LOSE,
            AllyMove::SCISSORS => MOVE_VALUE_SCISSORS + RESULT_VALUE_DRAW
        }
    }
}

fn get_perfect_game_rounds_from_file(file: &String) -> Vec<PerfectGameRound> {
    let mut results: Vec<PerfectGameRound> = Vec::new();
    for line in file.lines() {
        let enemy = line.chars().nth(0).expect("The input file appears to be broken");
        let outcome = line.chars().nth(2).expect("The input file appears to be broken");

        let enemy_move = match enemy {
            'A' => EnemyMove::ROCK,
            'B' => EnemyMove::PAPER,
            'C' => EnemyMove::SCISSORS,
            _ => panic!("The input file appears to be broken")
        };
        let required_outcome = match outcome {
            'X' => RequiredOutcome::LOSE,
            'Y' => RequiredOutcome::DRAW,
            'Z' => RequiredOutcome::WIN,
            _ => panic!("The input file appears to be broken")
        };
        results.push(PerfectGameRound {
            enemy_move,
            required_outcome
        })
    }
    return results;
}

fn determine_ally_move_for_outcome(round: &PerfectGameRound) -> AllyMove {
    return match round.enemy_move {
        EnemyMove::ROCK => match round.required_outcome {
            RequiredOutcome::LOSE => AllyMove::SCISSORS,
            RequiredOutcome::DRAW => AllyMove::ROCK,
            RequiredOutcome::WIN => AllyMove::PAPER
        },
        EnemyMove::PAPER => match round.required_outcome {
            RequiredOutcome::LOSE => AllyMove::ROCK,
            RequiredOutcome::DRAW => AllyMove::PAPER,
            RequiredOutcome::WIN => AllyMove::SCISSORS
        },
        EnemyMove::SCISSORS => match round.required_outcome {
            RequiredOutcome::LOSE => AllyMove::PAPER,
            RequiredOutcome::DRAW => AllyMove::SCISSORS,
            RequiredOutcome::WIN => AllyMove::ROCK
        }
    }
}

fn convert_perfect_round_to_normal_round(round: &PerfectGameRound) -> GameRound {
    return GameRound {
        enemy_move: round.enemy_move,
        ally_move: determine_ally_move_for_outcome(&round)
    }
}
