fn main() {
    let input = include_str!("input1.txt");
    print!("{}", process_part1(input))
}

fn process_part1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.split('\n') {
        if !line.is_empty() {
            sum += line
                .chars()
                .find(|x: &char| x.is_ascii_digit()).unwrap()
                .to_digit(10)
                .unwrap()
                * 10;
            sum += line
                .chars()
                .rfind(|x: &char| x.is_ascii_digit()).unwrap()
                .to_digit(10)
                .unwrap();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::process_part1;
    #[test]
    fn it_works_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process_part1(input), 142);
    }
}
