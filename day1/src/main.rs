use std::{fs::File, io::Read, path::Path};

mod part1;
mod part2;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn parse<F>(input: &str, parse_func: F) -> u32
where
    F: Fn(&str) -> u32,
{
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_func)
        .sum()
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let file_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let mut file = File::open(file_path).expect("Failed to open input.txt");
    let mut buf = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut buf)
        .expect("Failed to read file content");

    let result1 = parse(&buf, part1::parse);
    println!("Result 1: {result1}");

    let result2 = parse(&buf, part2::parse);
    println!("Result 2: {result2}");
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::{parse, part1, part2};

    #[test]
    fn parse_sample_pt1() {
        let input = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#;
        assert_eq!(parse(input, part1::parse), 142);
    }

    #[test]
    fn parse_sample_pt2() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;
        assert_eq!(parse(input, part2::parse), 281);
    }
}
