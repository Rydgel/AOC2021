use itertools::Itertools;
use reformation::Reformation;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

#[derive(Reformation, Debug)]
#[reformation(r"{x1},{y1} -> {x2},{y2}")]
struct VentLine {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn parse_input(input: &str) -> Vec<VentLine> {
    input
        .lines()
        .map(|s| VentLine::parse(s).expect("bad input"))
        .collect_vec()
}

fn num_overlapping<'a>(lines: impl Iterator<Item=&'a VentLine>) -> usize {
    let mut points = HashMap::new();
    for &VentLine{x1, y1, x2, y2} in lines {
      let dx = (x2 - x1).signum();
      let dy = (y2 - y1).signum();
      let (mut x, mut y) = (x1, y1);
      while (x, y) != (x2 + dx, y2 + dy) {
        *points.entry((x, y)).or_insert(0) += 1;
        x += dx;
        y += dy;
      }
    }
    points.values().filter(|&&n| n > 1).count()
  }

fn part1(input: &[VentLine]) -> usize {
    num_overlapping(input.iter().filter(|&VentLine{x1, y1, x2, y2}| x1 == x2 || y1 == y2))
}

fn part2(input: &[VentLine]) -> usize {
    num_overlapping(input.iter())
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
        assert_eq!(result, 5);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 12);
    }
}