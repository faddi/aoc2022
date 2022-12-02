use std::fs;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Game {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Game {
    fn lose(&self) -> Game {
        match *self {
            Game::Rock => Game::Paper,
            Game::Paper => Game::Scissors,
            Game::Scissors => Game::Rock,
        }
    }
    fn win(&self) -> Game {
        match *self {
            Game::Rock => Game::Scissors,
            Game::Paper => Game::Rock,
            Game::Scissors => Game::Paper,
        }
    }
    fn draw(&self) -> Game {
        match *self {
            Game::Rock => Game::Rock,
            Game::Paper => Game::Paper,
            Game::Scissors => Game::Scissors,
        }
    }
}

fn part2(file_contents: &str) {
    let mut score: u32 = 0;

    for line in file_contents.lines() {
        let spec: Vec<&str> = line.split(" ").collect();

        let p1 = match spec[0] {
            "A" => Game::Rock,
            "B" => Game::Paper,
            "C" => Game::Scissors,
            _ => panic!("Invalid input"),
        };

        let p2 = match spec[1] {
            "X" => p1.win(),
            "Y" => p1.draw(),
            "Z" => p1.lose(),
            _ => panic!("Invalid input"),
        };

        let game_result: u32 = match (p1, p2) {
            (Game::Rock, Game::Scissors) => 0,
            (Game::Paper, Game::Rock) => 0,
            (Game::Scissors, Game::Paper) => 0,
            (Game::Rock, Game::Paper) => 6,
            (Game::Paper, Game::Scissors) => 6,
            (Game::Scissors, Game::Rock) => 6,
            _ => 3,
        };

        score += game_result + p2 as u32;
    }

    println!("Score: {}", score);
}

fn part1(file_contents: &str) {
    let mut score: u32 = 0;

    for line in file_contents.lines() {
        let spec: Vec<&str> = line.split(" ").collect();

        let p1 = match spec[0] {
            "A" => Game::Rock,
            "B" => Game::Paper,
            "C" => Game::Scissors,
            _ => panic!("Invalid input"),
        };

        let p2 = match spec[1] {
            "X" => Game::Rock,
            "Y" => Game::Paper,
            "Z" => Game::Scissors,
            _ => panic!("Invalid input"),
        };

        let game_result: u32 = match (p1, p2) {
            (Game::Rock, Game::Scissors) => 0,
            (Game::Paper, Game::Rock) => 0,
            (Game::Scissors, Game::Paper) => 0,
            (Game::Rock, Game::Paper) => 6,
            (Game::Paper, Game::Scissors) => 6,
            (Game::Scissors, Game::Rock) => 6,
            _ => 3,
        };

        score += game_result + p2 as u32;
    }

    println!("Score: {}", score);
}

fn main() {
    let file_contents =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    part1(&file_contents);
    part2(&file_contents);
}
