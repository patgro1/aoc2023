use std::ops::Range;

use nom::IResult;

// struct Map V

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

    fn contains(self: &Self, val: u32) -> bool {
        self.source_range.contains(&val)
    }

    // fn get_mapping(
}

fn main() {
    let input = include_str!("input1.txt");
    println!("{}", process(input));
}

fn process(input: &str) -> u32 {
    0
}

// fn parse_map(input: &str) -> IResult<>

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn it_find_contained_num() {
        let map: SingleRangeMap = SingleRangeMap::new(10, 20, 30);
        assert!(!map.contains(10));
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
