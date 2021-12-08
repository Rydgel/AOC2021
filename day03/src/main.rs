use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| u32::from_str_radix(s, 2).expect("bad input"))
        .collect_vec()
}

fn width(input: &[u32]) -> u32 {
    input
        .iter()
        .map(|n| 32 - n.leading_zeros())
        .max()
        .unwrap()
}

fn common(input: &[u32], i: u32) -> u32 {
    let size_input = input.len() as f32;
    let sum: u32 = input
        .iter()
        .map(|n| n >> i & 1)
        .sum();
    
    ((sum as f32 / size_input) + 0.5).trunc() as u32
}

fn gamma(input: &[u32], width: u32) -> u32 {
    (0..width)
        .map(|i| common(input, i) << i)
        .sum()
}

fn epsilon(input: &[u32], width: u32) -> u32 {
    (0..width)
        .map(|i| (1 - common(input, i)) << i)
        .sum()
}

fn rating(input: &[u32], width: u32, most_common: u32) -> u32 {
    let mut i = width - 1;
    let mut diag = input.to_vec();

    while diag.len() > 1 {
        let bit = common(&diag, i) ^ most_common;
        diag = diag.into_iter().filter(|n| n >> i & 1 == bit).collect_vec();
        if i > 0 {
            i -= 1;
        }
    };

    *diag.first().unwrap()
}

fn oxygen(input: &[u32], width: u32) -> u32 {
    rating(input, width, 1)
}

fn co2_scrub(input: &[u32], width: u32) -> u32 {
    rating(input, width, 0)
}

fn part1(input: &[u32], width: u32) -> u32 {
    gamma(input, width) * epsilon(input, width)
}

fn part2(input: &[u32], width: u32) -> u32 {
    oxygen(input, width) * co2_scrub(input, width)
}

fn main() {
    let input = parse_input(INPUT);
    let width = width(&input);
    println!("day03 p1: {:?}", part1(&input, width));
    println!("day03 p2: {:?}", part2(&input, width));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let width = width(&input);
        let result = part1(&input, width);
        assert_eq!(result, 198);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let width = width(&input);
        let result = part2(&input, width);
        assert_eq!(result, 230);
    }
}