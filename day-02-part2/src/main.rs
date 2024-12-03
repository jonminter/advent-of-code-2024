use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() -> Result<(), Error> {
    let f = File::open("day-02-input.txt").map_err(Error::IoError)?;
    let result = find_num_safe_reports(f)?;
    println!("{}", result);

    Ok(())
}

fn find_num_safe_reports(f: File) -> Result<i32, Error> {
    let reader = BufReader::new(f);

    let num_safe = reader
        .lines()
        .map(|r| r.map_err(Error::IoError))
        .map(parse_into_numbers)
        .map(determine_if_removing_any_one_element_makes_report_safe)
        .try_fold(0, sum_safe_reports)?;
    Ok(num_safe)
}

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
    NotParseableAsNumber(std::num::ParseIntError),
}

fn parse_into_numbers(line: Result<String, Error>) -> Result<Vec<i32>, Error> {
    match line {
        Ok(line_str) => {
            let values: Vec<i32> = line_str
                .split_whitespace()
                .map(|v| v.parse())
                .collect::<Result<Vec<i32>, _>>()
                .map_err(Error::NotParseableAsNumber)?;
            Ok(values)
        }
        Err(e) => Err(e),
    }
}

fn determine_if_removing_any_one_element_makes_report_safe(
    maybe_report: Result<Vec<i32>, Error>,
) -> Result<bool, Error> {
    maybe_report.map(|report| {
        for i in 0..report.len() {
                let (a, b) = report.split_at(i);
                let one_dropped: Vec<i32> = a.iter().chain(b.iter().skip(1)).cloned().collect();
                if report_is_safe(one_dropped) {
                    return true;
                }
        }
        false
    })
}
fn report_is_safe(report: Vec<i32>) -> bool {
    let init_diff_set: HashSet<i32> = HashSet::new();
    let (all_diffs, all_valid) = report
        .windows(2)
        .map(|w| {
            let diff = w[0] - w[1];
            let abs_diff = diff.abs();
            let is_diff_in_range = abs_diff >= 1 && abs_diff <= 3;
            (diff, is_diff_in_range)
        })
        .fold(
            (init_diff_set, true),
            |(mut all_diffs, mut all_valid), (diff_dir, is_valid_range)| {
                all_diffs.insert(diff_dir);
                all_valid &= is_valid_range;

                (all_diffs, all_valid)
            },
        );

    let all_increasing = all_diffs.iter().all(|d| *d > 0);
    let all_decreasing = all_diffs.iter().all(|d| *d < 0);
    let is_safe = (all_increasing || all_decreasing) && all_valid;

    is_safe
}

fn sum_safe_reports(accum: i32, next_item: Result<bool, Error>) -> Result<i32, Error> {
    next_item.map(|is_safe| if is_safe { accum + 1 } else { accum + 0 })
}
