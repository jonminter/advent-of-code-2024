use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

fn main() -> Result<(), Error> {
    let f = File::open("day-01-input.txt").map_err(Error::IoError)?;
    let result = find_total_distance(f)?;
    println!("{}", result);

    Ok(())
}

fn find_total_distance(f: File) -> Result<i32, Error> {
    let reader = BufReader::new(f);

    let a_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let b_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    let (mut a_heap, mut b_heap) = reader
        .lines()
        .map(|r| r.map_err(Error::IoError))
        .map(parse_into_numbers)
        .try_fold((a_heap, b_heap), min_heap_accum)?;

    let a_sorted = std::iter::from_fn(|| a_heap.pop().map(|a| a.0));
    let b_sorted = std::iter::from_fn(|| b_heap.pop().map(|b| b.0));
    let result = a_sorted
        .zip(b_sorted)
        .map(calculate_distance)
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

fn min_heap_accum(
    heaps: (BinaryHeap<Reverse<i32>>, BinaryHeap<Reverse<i32>>),
    next_item: Result<(i32, i32), Error>,
) -> Result<(BinaryHeap<Reverse<i32>>, BinaryHeap<Reverse<i32>>), Error> {
    match (next_item, heaps) {
        (Ok((a, b)), (mut a_heap, mut b_heap)) => {
            a_heap.push(Reverse(a));
            b_heap.push(Reverse(b));
            Ok((a_heap, b_heap))
        }
        (Err(e), _) => Err(e),
    }
}

fn calculate_distance(numbers: (i32, i32)) -> i32 {
    match numbers {
        (a, b) => {
            (a - b).abs()
        }
    }
}
