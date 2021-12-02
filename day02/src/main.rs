use itertools::Itertools;
use reformation::Reformation;

const INPUT: &str = include_str!("../input.txt");

#[derive(Reformation, Debug)]
enum Instructions {
    #[reformation(r"forward {}")]
    Forward(usize),
    #[reformation(r"down {}")]
    Down(usize),
    #[reformation(r"up {}")]
    Up(usize),
}

fn parse_input(input: &str) -> Vec<Instructions> {
    input
        .lines()
        .map(|s| Instructions::parse(s).expect("Malformed instructions"))
        .collect_vec()
}


fn part1(input: &[Instructions]) -> usize {
    let mut position = 0;
    let mut depth = 0;

    for instruction in input {
        match &instruction {
            Instructions::Forward(n) => position += n,
            Instructions::Down(n) => depth += n,
            Instructions::Up(n) => depth -= n,
        }
    }

    position * depth
}

fn part2(input: &[Instructions]) -> usize {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in input {
        match &instruction {
            Instructions::Forward(n) => { position += n; depth += aim * n }
            Instructions::Down(n) => aim += n,
            Instructions::Up(n) => aim -= n,
        }
    }

    position * depth
}

fn main() {
    let input = parse_input(INPUT);
    println!("day1 p1: {:?}", part1(&input));
    println!("day1 p2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 150);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 900);
    }
}