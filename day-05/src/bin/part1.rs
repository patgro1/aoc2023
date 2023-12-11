use std::ops::Range;

use nom::{combinator::map, IResult, multi::separated_list1, character::complete::{multispace1, digit1, line_ending}};

#[derive(Debug, Eq, PartialEq)]
struct SingleRangeMap {
    dest_start: u32,
    source_range: Range<u32>,
}

impl SingleRangeMap {
    fn new(dest_start: u32, source_start: u32, range_len: u32) -> Self {
        SingleRangeMap {
            dest_start,
            source_range: (source_start..source_start + range_len),
        }
    }

    fn contains(&self, val: u32) -> bool {
        self.source_range.contains(&val)
    }

    fn get_mapping(&self, val: u32) -> u32 {
        val - self.source_range.start + self.dest_start

    }
}

#[derive(Debug, Eq, PartialEq)]
struct RangeMap {
    maps: Vec<SingleRangeMap>
}

impl RangeMap {
    fn new(maps: Vec<SingleRangeMap>) -> Self {
        RangeMap { maps }
    }

    fn get_mapping(&self, val: u32) -> u32 {
        if let Some(map) = self.maps.iter().find(|x| x.contains(val)) {
            map.get_mapping(val)
        } else{
            val
        }
    }
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{}", process(input));
}

fn process(input: &str) -> u32 {
    0
}

fn parse_range(input: &str) -> IResult<&str, SingleRangeMap> {
    println!("{input}");
    map(separated_list1(multispace1, map(digit1, |x: &str| x.parse::<u32>().unwrap())), |x| {
        SingleRangeMap::new(x[0], x[1], x[2])
    })(input)
}

fn parse_range_map(input: &str) -> IResult<&str, RangeMap> {
    map(separated_list1(line_ending, parse_range), |x| {
        RangeMap::new(x)
    })(input)
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn it_find_contained_num() {
        let map: SingleRangeMap = SingleRangeMap::new(10, 20, 30);
        assert!(!map.contains(10));
        assert!(!map.contains(50));
        assert!(map.contains(30));
        assert!(map.contains(41));
    }
    #[test]
    fn it_maps_num_in_single_range() {
        let map: SingleRangeMap = SingleRangeMap::new(50, 98, 10);
        assert_eq!(map.get_mapping(99), 51);
    }

    #[test]
    fn it_maps_in_multi_range() {
        let ranges = vec!(SingleRangeMap::new(50, 98, 2), SingleRangeMap::new(52, 50, 48));
        let map = RangeMap::new(ranges);
        assert_eq!(map.get_mapping(79), 81);
        assert_eq!(map.get_mapping(14), 14);
        assert_eq!(map.get_mapping(55), 57);
        assert_eq!(map.get_mapping(13), 13);
    }

    #[test]
    fn it_parses_range() {
        let input = "50 98 2";
        assert_eq!(parse_range(input), Ok(("", SingleRangeMap::new(50, 98, 2))));
    }

    #[test]
    fn it_parses_range_map() {
        let input = "50 98 2
52 50 48
10 20 3";
        assert_eq!(parse_range_map(input), Ok(("", RangeMap::new(vec!(SingleRangeMap::new(50,98,2), SingleRangeMap::new(52, 50, 48))))));
    }

    #[test]
    fn it_works_part1_optimized() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process(input), 35);
    }
}
