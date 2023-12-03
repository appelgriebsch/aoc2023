pub(crate) fn parse(line: &str) -> u32 {
    let start = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).expect("can't parse number"))
        .take(1)
        .next();
    let end = line
        .chars()
        .rev()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).expect("can't parse number"))
        .take(1)
        .next();
    match (start, end) {
        (Some(start), Some(end)) => start * 10 + end,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::part1::parse;

    #[test]
    fn parse_input1() {
        let input = "1abc2";
        assert_eq!(parse(input), 12);
    }

    #[test]
    fn parse_input2() {
        let input = "a1b2c3d4e5f";
        assert_eq!(parse(input), 15);
    }

    #[test]
    fn parse_input3() {
        let input = "treb7uchet";
        assert_eq!(parse(input), 77);
    }
}
