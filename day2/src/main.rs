use crate::game::{Draw, Game};
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod game;

fn parse_1(input: &str) -> u32 {
    let bag = Draw::new(14, 13, 12);
    let games = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Game::try_from(line).expect("Unable to parse game line"))
        .filter(|game| game.draws().iter().all(|draw| bag.can_contain(draw)))
        .collect::<Vec<_>>();
    games.iter().map(|game| game.id() as u32).sum()
}

fn parse_2(input: &str) -> u32 {
    let games = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Game::try_from(line).expect("Unable to parse game line"))
        .collect::<Vec<_>>();
    games.iter().fold(0, |acc, game| {
        let draw = game.max_draw();
        acc + draw.blue() as u32 * draw.green() as u32 * draw.red() as u32
    })
}

fn main() {
    let file_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let mut file = File::open(file_path).expect("Failed to open input.txt");
    let mut buf = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut buf)
        .expect("Failed to read file content");

    let result1 = parse_1(&buf);
    println!("Result 1: {result1}");

    let result2 = parse_2(&buf);
    println!("Result 2: {result2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_pt1() {
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        assert_eq!(parse_1(input), 8);
    }

    #[test]
    fn test_sample_pt2() {
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        assert_eq!(parse_2(input), 2286);
    }
}
