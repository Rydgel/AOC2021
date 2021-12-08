use itertools::Itertools;
use reformation::Reformation;
use cached::proc_macro::cached;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|s| i32::parse(s).expect("wrong input"))
        .collect_vec()
}

#[cached]
fn fish_stuff(a: i32) -> usize {
    if a > 0 {
        fish_stuff(a - 9) + fish_stuff(a - 7)
    } else {
        1
    }
}

fn part1(input: &[i32]) -> usize {
    input
        .iter()
        .map(|n| fish_stuff(80 - n))
        .sum()
}

fn part2(input: &[i32]) -> usize {
    input
        .iter()
        .map(|n| fish_stuff(256 - n))
        .sum()
}

fn main() {
    let input = parse_input(INPUT);
    println!("day06 p1: {:?}", part1(&input));
    println!("day06 p2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 5934);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 26984457539);
    }
}