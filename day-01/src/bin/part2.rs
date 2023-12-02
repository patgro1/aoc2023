fn main() {
    let input = include_str!("input2.txt");
    print!("{}", process_part2(input))
}

fn process_part2(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.split('\n') {
        if !line.is_empty() {
            let mut line = line.replace("one", "on1ne");
            line = line.replace("two", "tw2wo");
            line = line.replace("three", "thr3ree");
            line = line.replace("four", "fo4ur");
            line = line.replace("five", "fi5ve");
            line = line.replace("six", "si6ix");
            line = line.replace("seven", "se7ven");
            line = line.replace("eight", "ei8ght");
            line = line.replace("nine", "ni9ne");
            sum += line
                .chars()
                .find(|x: &char| x.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap()
                * 10;
            sum += line
                .chars()
                .rfind(|x: &char| x.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::process_part2;
    #[test]
    fn it_works_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        assert_eq!(process_part2(input), 281);
    }
}
