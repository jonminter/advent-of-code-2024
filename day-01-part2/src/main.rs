use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

fn main() -> Result<(), Error> {
    let f = File::open("day-01-input.txt").map_err(Error::IoError)?;
    let result = find_similarity_score(f)?;
    println!("{}", result);

    Ok(())
}

fn find_similarity_score(f: File) -> Result<i32, Error> {
    let reader = BufReader::new(f);

    let left_list: Vec<i32> = Vec::new();
    let right_occurrence_counter: OccurrenceCounter = OccurrenceCounter::new();

    let (left_list, right_occurence_counter) = reader
        .lines()
        .map(|r| r.map_err(Error::IoError))
        .map(parse_into_numbers)
        .try_fold(
            (left_list, right_occurrence_counter),
            accum_into_left_list_and_right_map,
        )?;

    let result = left_list
        .iter()
        .map(|left| left * right_occurence_counter.num_occurances(left))
        .fold(0, |a, b| a + b);
    Ok(result)
}

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
    NotParseableAsNumber(ParseIntError),
    WrongNumValuesToUnpack,
}

fn parse_into_numbers(line: Result<String, Error>) -> Result<(i32, i32), Error> {
    match line {
        Ok(line_str) => {
            let values: Vec<i32> = line_str
                .split_whitespace()
                .map(|v| v.parse())
                .collect::<Result<Vec<i32>, _>>()
                .map_err(Error::NotParseableAsNumber)?;
            if values.len() != 2 {
                Err(Error::WrongNumValuesToUnpack)
            } else {
                Ok((*values.get(0).unwrap(), *values.get(1).unwrap()))
            }
        }
        Err(e) => Err(e),
    }
}

struct OccurrenceCounter {
    number_counts: HashMap<i32, i32>,
}

impl OccurrenceCounter {
    fn new() -> Self {
        Self {
            number_counts: HashMap::new(),
        }
    }
    fn increment_observed_count(&mut self, n: i32) {
        self.number_counts
            .entry(n)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    fn num_occurances(&self, n: &i32) -> i32 {
        *self.number_counts.get(n).unwrap_or(&0)
    }
}

fn accum_into_left_list_and_right_map(
    accum: (Vec<i32>, OccurrenceCounter),
    next_item: Result<(i32, i32), Error>,
) -> Result<(Vec<i32>, OccurrenceCounter), Error> {
    match (next_item, accum) {
        (Ok((left, right)), (mut left_list, mut right_counts)) => {
            left_list.push(left);
            right_counts.increment_observed_count(right);
            Ok((left_list, right_counts))
        }
        (Err(e), _) => Err(e),
    }
}
