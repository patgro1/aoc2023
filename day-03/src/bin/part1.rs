fn main() {
    let input = include_str!("input1.txt");
    print!("{}", process(input))
}

fn process(input: &str) -> u32 {
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
    part_numbers.iter().sum()
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
        assert_eq!(process(input), 4361);
    }
}
