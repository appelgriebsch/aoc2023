use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct NumericToken(usize, u32);

impl Ord for NumericToken {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for NumericToken {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn parse(line: &str) -> u32 {
    let map = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let start = map
        .keys()
        .map(|key| {
            if let Some(pos) = line.find(*key) {
                NumericToken(pos, map[key])
            } else {
                NumericToken(usize::MAX, 0)
            }
        })
        .chain(line.find(char::is_numeric).map(|pos| {
            NumericToken(
                pos,
                line.chars()
                    .nth(pos)
                    .unwrap()
                    .to_digit(10)
                    .expect("can't parse number"),
            )
        }))
        .min();
    let end = map
        .keys()
        .map(|key| {
            if let Some(pos) = line.rfind(*key) {
                NumericToken(pos, map[key])
            } else {
                NumericToken(usize::MIN, 0)
            }
        })
        .chain(line.rfind(char::is_numeric).map(|pos| {
            NumericToken(
                pos,
                line.chars()
                    .nth(pos)
                    .unwrap()
                    .to_digit(10)
                    .expect("can't parse number"),
            )
        }))
        .max();

    match (start, end) {
        (Some(NumericToken(_, start_value)), Some(NumericToken(_, end_value))) => {
            start_value * 10 + end_value
        }
        _ => panic!("No numeric token found"),
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::part2::parse;

    #[test]
    fn parse_input1() {
        let input = "two1nine";
        assert_eq!(parse(input), 29);
    }

    #[test]
    fn parse_input2() {
        let input = "eightwothree";
        assert_eq!(parse(input), 83);
    }

    #[test]
    fn parse_input3() {
        let input = "4nineeightseven2";
        assert_eq!(parse(input), 42);
    }
}
