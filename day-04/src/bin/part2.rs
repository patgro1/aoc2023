use nom::{bytes::complete::tag, multi::separated_list1, character::complete::{digit1, space1, line_ending}, IResult, combinator::{map,map_res}, sequence::{separated_pair, pair}};
use sorted_vec::SortedVec;

fn main() {
    let input = include_str!("input1.txt");
    println!("{}", process(input));
}

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    index: u32,
    winning_numbers: SortedVec<u32>,
    played_numbers: SortedVec<u32>,
    copies: u32
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
    let mut cards = parse_input(input).unwrap().1;

    let mut card_copies: Vec<u32> = vec![1; cards.len()];
    for card in &mut cards {
        let intersect_size: u32 = calculate_intersect(card.winning_numbers.clone(), card.played_numbers.clone()).len().try_into().unwrap();
        let current_card_copies = card_copies[card.index as usize];
        for winned_card in &mut card_copies[(card.index+1).try_into().unwrap()..=(card.index+intersect_size).try_into().unwrap()] {
            *winned_card += current_card_copies;
        }

    }
    card_copies.iter().sum()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, parse_card)(input)

}

fn parse_numbers(input: &str) -> IResult<&str, SortedVec<u32>> {
    map(separated_list1(space1, map_res(digit1, |x: &str| x.parse())), |x: Vec<u32>| SortedVec::from_unsorted(x))(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map(separated_pair(separated_pair(tag("Card"), space1, digit1), pair(tag(":"), space1), separated_pair(parse_numbers, pair(tag(" |"), space1), parse_numbers)), |x| {
        Card{index:x.0 .1.parse::<u32>().unwrap()-1, winning_numbers:x.1.0, played_numbers:x.1.1, copies:1}
    }
    )(input)

}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn it_works_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input), 30);
    }
}
