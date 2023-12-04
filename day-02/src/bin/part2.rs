use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::IResult;

static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;

#[derive(Debug, PartialEq)]
pub struct Game {
    index: usize,
    draws: Vec<Draw>,
}

#[derive(Debug, PartialEq)]
pub struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq)]
pub enum Rocks {
    Red(u32),
    Blue(u32),
    Green(u32),
}

impl Rocks {
    fn new(color: &str, quantity: u32) -> Rocks {
        match color {
            "red" => Rocks::Red(quantity),
            "blue" => Rocks::Blue(quantity),
            "green" => Rocks::Green(quantity),
            _ => panic!("Unexpected rock color"),
        }
    }
}

fn main() {
    let input = include_str!("input2.txt");
    print!("{}", process(input))
}

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    nom::multi::separated_list0(nom::character::complete::newline, parse_game)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    nom::combinator::map(
        nom::sequence::separated_pair(
            nom::sequence::pair(
                tag("Game "),
                nom::combinator::map_res(nom::character::complete::digit1, |x: &str| x.parse()),
            ),
            tag(": "),
            parse_draws,
        ),
        |x| Game {
            index: x.0 .1,
            draws: x.1,
        },
    )(input)
}

fn parse_rock(input: &str) -> IResult<&str, Rocks> {
    nom::combinator::map(
        separated_pair(
            nom::combinator::map_res(nom::character::complete::digit1, |x: &str| x.parse()),
            tag(" "),
            nom::character::complete::alpha1,
        ),
        |x| Rocks::new(x.1, x.0),
    )(input)
}

fn parse_draws(input: &str) -> IResult<&str, Vec<Draw>> {
    nom::multi::separated_list1(tag("; "), parse_draw)(input)
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    nom::combinator::map(nom::multi::separated_list1(tag(", "), parse_rock), |x| {
        let mut draw: Draw = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };
        for rock in x {
            match rock {
                Rocks::Red(qty) => draw.red = qty,
                Rocks::Blue(qty) => draw.blue = qty,
                Rocks::Green(qty) => draw.green = qty,
            };
        }
        draw
    })(input)
}

fn process(input: &str) -> u32 {
    let games = parse(input).unwrap().1;

    games
        .iter()
        .map(|game| {
            let max_red = game.draws.iter().map(|x| x.red).max().unwrap_or(0);
            let max_blue = game.draws.iter().map(|x| x.blue).max().unwrap_or(0);
            let max_green = game.draws.iter().map(|x| x.green).max().unwrap_or(0);
            max_red * max_blue * max_green
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game_parser() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            parse_game(input),
            Ok((
                "",
                Game {
                    index: 1,
                    draws: vec!(
                        Draw {
                            red: 4,
                            green: 0,
                            blue: 3
                        },
                        Draw {
                            red: 1,
                            green: 2,
                            blue: 6
                        },
                        Draw {
                            red: 0,
                            green: 2,
                            blue: 0
                        }
                    )
                }
            ))
        );
    }
    #[test]
    fn test_rock_parser() {
        let input = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            parse_draws(input),
            Ok((
                "",
                vec!(
                    Draw {
                        red: 4,
                        blue: 3,
                        green: 0
                    },
                    Draw {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Draw {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                )
            ))
        );
    }
    #[test]
    fn test_draws_parser() {
        let input = "3 blue, 4 red";
        assert_eq!(
            parse_draw(input),
            Ok((
                "",
                Draw {
                    red: 4,
                    green: 0,
                    blue: 3
                }
            ))
        );
    }

    #[test]
    fn test_draw_parser() {
        let input = "3 blue, 4 red";
        assert_eq!(
            parse_draw(input),
            Ok((
                "",
                Draw {
                    red: 4,
                    green: 0,
                    blue: 3
                }
            ))
        );
    }

    #[test]
    fn it_works_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(process(input), 2286);
    }
}
