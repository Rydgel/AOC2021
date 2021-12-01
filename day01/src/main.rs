const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}


fn part1(input: &[usize]) -> usize {
    input
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

fn part2(input: &[usize]) -> usize {
    input
        .windows(4)
        .filter(|w| w[0] < w[3])
        .count()
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
        assert_eq!(result, 7);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 5);
    }
}