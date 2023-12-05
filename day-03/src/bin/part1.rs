use std::cmp::{max, min};

fn main() {
    let input = include_str!("input1.txt");
    // println!("{}", process_brute_force(input));
    println!("{}", process(input));
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
                // println!("Found digit {chr} at line {line_idx}, idx {c_idx}");
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
                        if n_ch != '.' {
                            symbols.push(Point {
                                line: line_idx,
                                char: c_idx,
                            });
                        }
                        break;
                    }
                    // Peek if the next char is existing and not a digit
                    // let next_possible = line_iter.peek();
                    // println!("Next possible ch: {:?}", next_possible);
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
            } else if chr != '.' {
                symbols.push(Point {
                    line: line_idx,
                    char: c_idx,
                });
                c_idx += 1;
                // println!("Found a symbol at line {line_idx}, idx {c_idx}");
            } else {
                c_idx += 1;
            }
        }
    }
    // println!("List of all numbers: {:?}", numbers);
    // println!("Symbols: {:?}", symbols);
    let mut part_numbers: Vec<u32> = vec![];
    // println!("Line length: {line_length}");
    for possible_part in numbers {
        // println!("Checking possible part {:?}", possible_part);
        // This should never fail because we are capping the minimum at 0
        let min_line: usize = max(possible_part.line as i32 - 1, 0).try_into().unwrap();
        let max_line: usize = min(possible_part.line + 1, input.lines().count() - 1);
        // This should never fail because we are capping the minimum at 0
        let min_idx: usize = max(possible_part.start as i32 - 1, 0).try_into().unwrap();
        let max_idx: usize = min(possible_part.end + 1, line_length - 1);
        // println!("Will check the line {min_line}->{max_line}, idx {min_idx}->{max_idx}");
        // Up
        let adjacent_symbols: Vec<_> = symbols
            .iter()
            .filter(|x| min_line <= x.line && x.line <= max_line)
            .filter(|x| min_idx <= x.char && x.char <= max_idx)
            .collect();
        // println!("Adjacent symbols: {:?}", adjacent_symbols);
        if !adjacent_symbols.is_empty() {
            // println!("Will count number {:?}", possible_part);
            part_numbers.push(possible_part.number);
        }
    }
    // println!("Part numbers: {:?}", part_numbers);
    part_numbers.iter().sum()
}

fn process_brute_force(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in lines {
        let mut line: Vec<_> = line.chars().collect();
        line.insert(0, '.');
        line.push('.');
        matrix.push(line)
    }
    matrix.push(vec!['.'; matrix[0].len()]);
    matrix.insert(0, vec!['.'; matrix[0].len()]);

    let mut part_numbers: Vec<u32> = vec![];

    for (current_line, line) in matrix.iter().enumerate() {
        let mut current_char = 0;
        let mut line_iter = line.iter();
        // println!("{:?}", line);
        while let Some(ch) = line_iter.next() {
            if ch.is_ascii_digit() {
                let starting_char = current_char;
                let mut ending_char = current_char;
                let mut number: u32 = ch.to_digit(10).unwrap();
                for n_ch in line_iter.by_ref() {
                    current_char += 1;
                    if n_ch.is_ascii_digit() {
                        ending_char += 1;
                        number *= 10;
                        number += n_ch.to_digit(10).unwrap();
                    } else {
                        // println!(
                        //     "Found a number at {starting_line}, {starting_char} => {ending_char}"
                        // );
                        //Check if the number is adjacent to a symbol
                        if !line[starting_char - 1].is_ascii_digit()
                            && line[starting_char - 1] != '.'
                        {
                            // println!("{number} is a valid part number");
                            part_numbers.push(number)
                        }
                        if !line[ending_char + 1].is_ascii_digit() && line[ending_char + 1] != '.' {
                            // println!("{number} is a valid part number");
                            part_numbers.push(number)
                        }
                        if matrix[current_line - 1][starting_char - 1..=ending_char + 1]
                            .iter()
                            .filter(|x| !x.is_ascii_digit() && **x != '.')
                            .count()
                            > 0
                        {
                            // println!("{number} is a valid part number");
                            part_numbers.push(number)
                        }
                        if matrix[current_line + 1][starting_char - 1..=ending_char + 1]
                            .iter()
                            .filter(|x| !x.is_ascii_digit() && **x != '.')
                            .count()
                            > 0
                        {
                            // println!("{number} is a valid part number");
                            part_numbers.push(number)
                        }

                        break;
                    }
                }
            }
            current_char += 1
        }
    }
    println!("Part numbers: {:?}", part_numbers);
    part_numbers.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn it_works_part1_brute_force() {
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
        assert_eq!(process_brute_force(input), 4361);
    }
    #[test]
    fn it_works_part1_optimized() {
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
        assert_eq!(process(input), 4361);
    }
}
