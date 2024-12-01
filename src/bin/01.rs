use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    // println!("=== Part 1 ===");
    // let (mut c1,mut c2):(Vec<i32>,Vec<i32>) = (Vec::new(),Vec::new());
    // let l: Vec<_> =TEST.lines()
    //     .map(|l| {
    //         l.split_whitespace().map(|char|{
    //             char.parse::<i32>().unwrap()
    //         }).collect::<Vec<i32>>()
    //
    //
    //     }).collect();
    // l.iter().for_each(|pair|{
    //     c1.push(pair[0]); c2.push(pair[1]);
    //     c1.sort_unstable(); c2.sort_unstable();
    // });
    // let diff: Vec<_> = c1.iter().zip(c2.iter()).map(|(a,b)| (a - b).abs()).collect();
    // let total = diff.iter().sum::<i32>();
    // println!("l {:?}",l);
    // println!("c1: {:?} | c2: {:?}",c1,c2);
    // println!("diff {:?}",diff);
    // println!("total {:?}",total);


    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // let answer = reader.lines().flatten().count();
        let (mut c1,mut c2):(Vec<i32>,Vec<i32>) = (Vec::new(),Vec::new());
        let l: Vec<_> =reader.lines()
            .map(|l| {
                l.unwrap().split_whitespace().map(|char|{
                    char.parse::<i32>().unwrap()
                }).collect::<Vec<i32>>()


            }).collect();
        l.iter().for_each(|pair|{
            c1.push(pair[0]); c2.push(pair[1]);
            c1.sort_unstable(); c2.sort_unstable();
        });
        let diff: Vec<_> = c1.iter().zip(c2.iter()).map(|(a,b)| (a - b).abs()).collect();
        let total = diff.iter().sum::<i32>();


        // println!("Final parsed data: {:?}", x);

        Ok(total as usize)

        // Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    // let c1_count_in_c2: Vec<i32> = c1.clone().into_iter().map(|e1|{
    //     println!("e1 is {}",e1);
    //     c2.clone().into_iter().filter(|&e2| e2 == e1).count() as i32
    // }).collect();
    // let c1_and_count: Vec<_> = c1.iter().zip(c1_count_in_c2.iter()).collect();
    // println!("c1_count_in_c2 {:?}",c1_and_count);
    // let simmilarities_sum = c1_and_count.iter().map(|(a,b)|{
    //     *a**b
    // }).collect::<Vec<i32>>().iter().sum::<i32>();
    // println!("{:?}", simmilarities_sum);

    //
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (mut c1,mut c2):(Vec<i32>,Vec<i32>) = (Vec::new(),Vec::new());
        let l: Vec<_> =reader.lines()
            .map(|l| {
                l.unwrap().split_whitespace().map(|char|{
                    char.parse::<i32>().unwrap()
                }).collect::<Vec<i32>>()


            }).collect();
        l.iter().for_each(|pair|{
            c1.push(pair[0]); c2.push(pair[1]);
            c1.sort_unstable(); c2.sort_unstable();
        });
        let c1_count_in_c2: Vec<i32> = c1.clone().into_iter().map(|e1|{
            // println!("e1 is {}",e1);
            c2.clone().into_iter().filter(|&e2| e2 == e1).count() as i32
        }).collect();
        let c1_and_count: Vec<_> = c1.iter().zip(c1_count_in_c2.iter()).collect();
        // println!("c1_count_in_c2 {:?}",c1_and_count);
        let simmilarities_sum = c1_and_count.iter().map(|(a,b)|{
            *a**b
        }).collect::<Vec<i32>>().iter().sum::<i32>();
        println!("{:?}", simmilarities_sum);
        Ok(simmilarities_sum as usize)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
