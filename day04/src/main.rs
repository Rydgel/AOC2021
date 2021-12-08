use vectrix::Matrix;

const INPUT: &str = include_str!("../input.txt");

type Board = Matrix<(i64, bool), 5, 5>;

fn parse_input(input: &str) -> (Vec<i64>, Vec<Board>) {
    let (draws, boards) = input.split_once("\n\n").unwrap();

    let draws = draws
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let boards = boards
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .flat_map(|line| {
                    line.split_whitespace()
                        .map(str::parse)
                        .map(Result::unwrap)
                        .map(|n| (n, false))
                })
                .collect()
        })
        .collect();
    (draws, boards)
}

fn update(board: Board, d: i64) -> Board {
    board
        .into_iter()
        .map(|(n, marked)| (n, marked || n == d))
        .collect()
}

fn is_win(board: &Board) -> bool {
    let row_win = board
        .iter_rows()
        .any(|row| row.iter().all(|(_, marked)| *marked));
    let col_win = board
        .iter_columns()
        .any(|col| col.iter().all(|(_, marked)| *marked));
    row_win || col_win
}

fn score(board: &Board, d: i64) -> i64 {
    let sum: i64 = board
        .iter()
        .filter(|(_, marked)| !marked)
        .map(|(n, _)| n)
        .sum();
    sum * d
}

fn part1((draws, mut boards): (Vec<i64>, Vec<Board>)) -> i64 {
    for d in draws {
        for board in boards.iter_mut() {
            *board = update(*board, d);
            if is_win(board) {
                return score(board, d);
            }
        }
    }
    unreachable!()
}

fn part2((draws, mut boards): (Vec<i64>, Vec<Board>)) -> i64 {
    for d in draws {
        if let [board] = *boards {
            return score(&update(board, d), d);
        }
        boards = boards
            .into_iter()
            .map(|board| update(board, d))
            .filter(|board| !is_win(board))
            .collect();
    }
    unreachable!()
}

fn main() {
    let input = parse_input(INPUT);
    println!("day1 p1: {:?}", part1(input.clone()));
    println!("day1 p2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input = parse_input(TEST_INPUT);
        let result = part1(input);
        assert_eq!(result, 4512);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = part2(input);
        assert_eq!(result, 230);
    }
}