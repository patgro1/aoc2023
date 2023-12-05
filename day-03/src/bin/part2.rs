use std::cmp::{max, min};

fn main() {
    let input = include_str!("input1.txt");
    print!("{}", process(input))
}

#[derive(Debug)]
pub struct Point {
    line: usize,
    char: usize,
}

#[derive(Debug)]
pub struct PartNum {
    number: u32,
    line: usize,
    start: usize,
    end: usize,
}

fn process(input: &str) -> u32 {
    let mut symbols: Vec<Point> = vec![];
    let mut numbers: Vec<PartNum> = vec![];
    let mut line_length: usize = 0;
    for (line_idx, line) in input.lines().enumerate() {
        line_length = line.len();
        let mut c_idx: usize = 0;
        let mut line_iter = line.chars().peekable();
        while let Some(chr) = line_iter.next() {
            if chr.is_ascii_digit() {
                let mut end_idx = c_idx;
                let start_idx = c_idx;
                let mut number: u32 = chr.to_digit(10).unwrap();
                while let Some(n_ch) = line_iter.by_ref().next() {
                    c_idx += 1;
                    if n_ch.is_ascii_digit() {
                        end_idx += 1;
                        number *= 10;
                        number += n_ch.to_digit(10).unwrap()
                    } else {
                        numbers.push(PartNum {
                            number,
                            line: line_idx,
                            start: start_idx,
                            end: end_idx,
                        });
                        // Special case if we have a symbol
                        if n_ch == '*' {
                            symbols.push(Point {
                                line: line_idx,
                                char: c_idx,
                            });
                        }
                        break;
                    }
                    // Peek if the next char is existing and not a digit
                    // let next_possible = line_iter.peek();
                    if let Some(next_possible) = line_iter.by_ref().peek() {
                        if !next_possible.is_ascii_digit() {
                            numbers.push(PartNum {
                                number,
                                line: line_idx,
                                start: start_idx,
                                end: end_idx,
                            });
                            break;
                        }
                    } else {
                        numbers.push(PartNum {
                            number,
                            line: line_idx,
                            start: start_idx,
                            end: end_idx,
                        });
                        break;
                    }
                }
                c_idx += 1;
            } else if chr == '*' {
                symbols.push(Point {
                    line: line_idx,
                    char: c_idx,
                });
                c_idx += 1;
            } else {
                c_idx += 1;
            }
        }
    }
    let mut gear_ratios: Vec<u32> = vec![];
    for possible_gear in symbols {
        // This should never fail because we are capping the minimum at 0
        let min_line: usize = max(possible_gear.line as i32 - 1, 0).try_into().unwrap();
        let max_line: usize = min(possible_gear.line + 1, input.lines().count() - 1);
        // This should never fail because we are capping the minimum at 0
        let min_idx: usize = max(possible_gear.char as i32 - 1, 0).try_into().unwrap();
        let max_idx: usize = min(possible_gear.char + 1, line_length - 1);
        // Up
        let adjacent_numbers: Vec<_> = numbers
            .iter()
            .filter(|x| min_line <= x.line && x.line <= max_line)
            .filter(|x| min_idx <= x.end && x.start <= max_idx)
            .collect();
        if adjacent_numbers.len() == 2 {
            gear_ratios.push(adjacent_numbers[0].number * adjacent_numbers[1].number);
        }
    }
    gear_ratios.iter().sum()
}


#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn it_works_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process(input), 467835);
    }
}
