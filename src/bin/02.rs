use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; //
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = get_lines(reader);
        println!("{:?}", lines);
        let row_with_diff_from_1_to_3 = lines
            .iter()
            .filter(|row| {
                // decrease or increase by from 1 to 3 and always increase or always decrease
                row.windows(2).all(|pair| {
                    let diff = (pair[1] - pair[0]).abs();

                    diff <= 3 && diff >= 1
                })
            })
            .collect::<Vec<_>>();
        println!("row with diff from 1 to 3 {:?}", row_with_diff_from_1_to_3);
        let row_with_acc_bigger_than_0 = row_with_diff_from_1_to_3
            .iter()
            .filter(|row| {
                row.windows(2).all(|pair| pair[1] - pair[0] >= 1)
                    || row.windows(2).all(|pair| pair[1] - pair[0] <= -1)
            })
            .collect::<Vec<_>>();
        println!(
            "row with acc bigger than 0 {:?}",
            row_with_acc_bigger_than_0
        );
        Ok(row_with_acc_bigger_than_0.iter().count())

        // let answer = reader.lines().flatten().count();
        // println!("Hello world");
        // Ok(answer)
    }




    // println!("{:?}", part1(BufReader::new(TEST.as_bytes())).unwrap());


    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    fn get_lines<R: BufRead>(reader: R) -> Vec<Vec<i32>> {
        reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .split_whitespace()
                    .map(|char| {
                        // parse as number
                        char.parse::<i32>().unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
    // this fn is created based on parted 1
    fn generate_subarrays_with_one_removed(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for i in 0..arr.len() {
            let mut subarray = arr.clone();
            subarray.remove(i);
            result.push(subarray);
        }
        result
    }
    fn is_line_suitable(row: Vec<i32>) -> bool {
        row.windows(2).all(|pair| {
            let diff = (pair[1] - pair[0]).abs();
            diff <= 3 && diff >= 1
        }) && (row.windows(2).all(|pair| pair[1] - pair[0] >= 1)
            || row.windows(2).all(|pair| pair[1] - pair[0] <= -1))
    }
    fn is_line_suitable_with_tolerant_1_unit(row: &Vec<i32>) -> bool {
        if is_line_suitable(row.clone()) {
            return true;
        } else {
            let sub_arrays = generate_subarrays_with_one_removed(row.clone());
            return sub_arrays.iter().any(|sub_array| is_line_suitable(sub_array.clone()))
        }

    }
    // println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = get_lines(reader);
        let suitable_line:Vec<Vec<i32>> = lines.iter().filter(|line|{
            is_line_suitable_with_tolerant_1_unit(line.clone())
        }).cloned().collect::<Vec<Vec<_>>>();
        // println!("suitable row {:?}",suitable_line);
        Ok(suitable_line.len())

    }
    // println!("part 2 with test {:?}",part2(BufReader::new(TEST.as_bytes()))?);
    //
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
