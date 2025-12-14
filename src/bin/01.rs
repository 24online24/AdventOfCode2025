use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines()
            .map(Result::unwrap)
            .fold((50, 0), |(dial, zero_count), line| {
                let (direction, magnitude_str) = line.split_at(1);
                let magnitude: i32 = magnitude_str.parse().unwrap();
                let new_dial = match direction {
                    "L" => (dial - magnitude).rem_euclid(100),
                    "R" => (dial + magnitude).rem_euclid(100),
                    _ => unreachable!(),
                };
                (new_dial, zero_count + (new_dial == 0) as usize)
            })
            .1)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(reader
            .lines()
            .map(Result::unwrap)
            .fold((50, 0), |(dial, zero_count), line| {
                let (direction, magnitude_str) = line.split_at(1);
                let magnitude: i32 = magnitude_str.parse().unwrap();

                match direction {
                    "L" => (
                        (dial - magnitude).rem_euclid(100),
                        zero_count
                            + (magnitude / 100
                                + if dial > 0 && magnitude % 100 >= dial {
                                    1
                                } else {
                                    0
                                }) as usize,
                    ),
                    "R" => (
                        (dial + magnitude).rem_euclid(100),
                        zero_count + (dial + magnitude) as usize / 100,
                    ),
                    _ => unreachable!(),
                }
            })
            .1)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
