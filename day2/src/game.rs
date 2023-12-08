use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Game {
    id: u8,
    draws: Vec<Draw>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Draw {
    blue: u8,
    green: u8,
    red: u8,
}

impl Game {
    fn new(id: u8, draws: &[Draw]) -> Self {
        Game {
            id,
            draws: draws.to_vec(),
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn draws(&self) -> &[Draw] {
        &self.draws
    }
}

impl Draw {
    pub fn new(blue: u8, green: u8, red: u8) -> Self {
        Draw { blue, green, red }
    }
    pub fn can_contain(&self, other: &Self) -> bool {
        self.blue >= other.blue && self.green >= other.green && self.red >= other.red
    }
}

impl TryFrom<&str> for Draw {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let parts = line.split(", ");
        let mut color_map = HashMap::new();
        parts.for_each(|part| {
            let (count, color) = part.split_at(part.find(' ').unwrap());
            color_map.insert(
                color.trim(),
                count
                    .parse::<u8>()
                    .map_err(|_| "Unable to parse draw")
                    .unwrap(),
            );
        });

        Ok(Draw::new(
            color_map.get("blue").unwrap_or(&0).to_owned(),
            color_map.get("green").unwrap_or(&0).to_owned(),
            color_map.get("red").unwrap_or(&0).to_owned(),
        ))
    }
}

impl TryFrom<&str> for Game {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut parts = line.trim().split(": ");
        let id = parts
            .next()
            .ok_or_else(|| "Unable to parse game id".to_string())?
            .split(' ')
            .nth(1)
            .ok_or_else(|| "Unable to parse game id".to_string())?
            .parse::<u8>()
            .map_err(|_| "Unable to parse game id".to_string())?;
        let draws = parts
            .next()
            .ok_or_else(|| "Unable to parse draws".to_string())?
            .split("; ")
            .map(Draw::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game::new(id, &draws))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_1() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::try_from(line).expect("Unable to parse line");
        assert_eq!(game.id, 1);
        assert_eq!(game.draws.len(), 3);
        assert_eq!(game.draws[0].blue, 3);
        assert_eq!(game.draws[0].green, 0);
        assert_eq!(game.draws[0].red, 4);
        assert_eq!(game.draws[1].blue, 6);
        assert_eq!(game.draws[1].green, 2);
        assert_eq!(game.draws[1].red, 1);
        assert_eq!(game.draws[2].blue, 0);
        assert_eq!(game.draws[2].green, 2);
        assert_eq!(game.draws[2].red, 0);
    }

    #[test]
    fn test_game_4() {
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let game = Game::try_from(line).expect("Unable to parse line");
        assert_eq!(game.id, 4);
        assert_eq!(game.draws.len(), 3);
        assert_eq!(game.draws[0].blue, 6);
        assert_eq!(game.draws[0].green, 1);
        assert_eq!(game.draws[0].red, 3);
        assert_eq!(game.draws[1].blue, 0);
        assert_eq!(game.draws[1].green, 3);
        assert_eq!(game.draws[1].red, 6);
        assert_eq!(game.draws[2].blue, 15);
        assert_eq!(game.draws[2].green, 3);
        assert_eq!(game.draws[2].red, 14);
    }

    #[test]
    fn test_draw_comparison_1() {
        let bag = Draw::new(14, 13, 12);
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = Game::try_from(line).expect("Unable to parse line");
        let result = game.draws.iter().all(|draw| bag.can_contain(draw));
        assert_eq!(result, false);
    }

    #[test]
    fn test_draw_comparison_2() {
        let bag = Draw::new(14, 13, 12);
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let game = Game::try_from(line).expect("Unable to parse line");
        let result = game.draws.iter().all(|draw| bag.can_contain(draw));
        assert_eq!(result, false);
    }

    #[test]
    fn test_draw_comparison_3() {
        let bag = Draw::new(14, 13, 12);
        let line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let game = Game::try_from(line).expect("Unable to parse line");
        let result = game.draws.iter().all(|draw| bag.can_contain(draw));
        assert_eq!(result, true);
    }
}
