use nom::{bytes::complete::tag, multi::separated_list1, character::complete::{digit1, space1, line_ending}, IResult, combinator::{map,map_res}, sequence::{separated_pair, pair}};
use sorted_vec::SortedVec;

fn main() {
    let input = include_str!("input1.txt");
    println!("{}", process(input));
}

#[derive(Debug, PartialEq)]
pub struct Card {
    index: usize,
    winning_numbers: SortedVec<u32>,
    played_numbers: SortedVec<u32>,
}

fn calculate_intersect(a: SortedVec<u32>, b: SortedVec<u32>) -> Vec<u32> {
    let mut intersec: Vec<u32> = vec![];
    let iter_a = a.iter();
    let mut iter_b = b.iter();
    if let Some(mut elem_b) = iter_b.next() {
        for elem_a in iter_a {
            while elem_b <= elem_a {
                if elem_a == elem_b {
                    intersec.push(*elem_b)
                }
                if let Some(n) = iter_b.next() {
                    elem_b = n;
                } else {
                    break;
                }
            }
        }
    }
    intersec

}

fn process(input: &str) -> u32 {
    let cards = parse_input(input).unwrap().1;

    let t: Vec<_> = cards.iter().map(|x| {
        calculate_intersect(x.played_numbers.clone(), x.winning_numbers.clone()).len()
    }).filter(|x|x > &0)
    .map(|x| 1 << (x-1)).collect();

    t.iter().sum()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_card)(input)

}

fn parse_numbers(input: &str) -> IResult<&str, SortedVec<u32>> {
    map(separated_list1(space1, map_res(digit1, |x: &str| x.parse())), |x: Vec<u32>| SortedVec::from_unsorted(x))(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map(separated_pair(separated_pair(tag("Card"), space1, digit1), pair(tag(":"), space1), separated_pair(parse_numbers, pair(tag(" |"), space1), parse_numbers)), |x| {
        Card{index:x.0 .1.parse().unwrap(), winning_numbers:x.1.0, played_numbers:x.1.1}
    }
    )(input)

}

#[cfg(test)]
mod test {
    use crate::*;
    use sorted_vec::SortedVec;
    #[test]
    fn it_calculates_intersection() {
        let a = SortedVec::from_unsorted(vec![17, 41, 48, 83, 86]);
        let b = SortedVec::from_unsorted(vec![6, 9, 17, 31, 48, 53, 83, 86]);
        assert_eq!(
            calculate_intersect(a, b),
            vec![17, 48, 83, 86]);
    }
    #[test]
    fn it_parses_numbers_list() {
        let input = "41 48 83 86 17";
        assert_eq!(
            parse_numbers(input),
            Ok(("", SortedVec::from_unsorted(vec![17, 41, 48, 83, 86])))
        );
    }

    #[test]
    fn it_parses_card() {
        let input = "Card 2: 13 32 20 16  1 | 61 30 68 82 17 32 24 19";
        assert_eq!(
            parse_card(input),
            Ok(("", Card{
                index: 2,
                winning_numbers: SortedVec::from_unsorted(vec![13,16,20,32,1]),
                played_numbers: SortedVec::from_unsorted(vec![17, 19, 24, 30, 32, 61, 68, 82])
            })));
    }
    #[test]
    fn it_works_part1_optimized() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input), 13);
    }
}
