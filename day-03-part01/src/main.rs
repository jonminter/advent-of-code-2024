use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn consume_remainder(input: &str) -> &str {
    let (_, no_remainder) = input.split_at(input.len());
    no_remainder
}

fn scan_for_next_token_and_consume<'a>(token: &str, input: &'a str) -> Result<&'a str, &'a str> {
    input.split_once(token).map_or_else(
        || Err(consume_remainder(input)),
        |(_, remaining)| Ok(remaining),
    )
}

fn consume_token<'a>(token: &str, input: &'a str) -> Result<&'a str, &'a str> {
    if input.len() < token.len() {
        return Err(consume_remainder(input));
    }

    let (possible_match, remainder) = input.split_at(token.len());
    if possible_match == token {
        Ok(remainder)
    } else {
        Err(input)
    }
}

fn read_digits<'a>(input: &str) -> Result<(i32, &str), &str> {
    let mut buf = String::new();
    for c in input.chars() {
        if c.is_numeric() {
            buf.push(c);
        } else {
            break;
        }
    }

    if buf.len() > 0 {
        let (_, remainder) = input.split_at(buf.len());
        Ok((buf.parse().unwrap(), remainder))
    } else {
        Err(input)
    }
}

fn consume_open_paren(remaining: &str) -> Result<&str, &str> {
    consume_token("(", remaining)
}

fn consume_comma(input: (i32, &str)) -> Result<(i32, &str), &str> {
    let (multiplicand, remaining) = input;

    consume_token(",", remaining).map(|r| (multiplicand, r))
}

fn read_second_number(input: (i32, &str)) -> Result<((i32, i32), &str), &str> {
    let (multiplicand, remaining) = input;
    read_digits(remaining).map(|(multiplier, r)| ((multiplicand, multiplier), r))
}

fn consume_close_paren(input: ((i32, i32), &str)) -> Result<((i32, i32), &str), &str> {
    let (multiplication, remaining) = input;
    consume_token(")", remaining).map(|r| (multiplication, r))
}

fn sum_multiplications<'a>(input: &'a str) -> Result<i32, String> {
    let mut remaining = input;
    let mut result = 0;
    while remaining.len() > 0 {
        let parse_result = scan_for_next_token_and_consume("mul", remaining)
            .and_then(consume_open_paren)
            .and_then(read_digits)
            .and_then(consume_comma)
            .and_then(read_second_number)
            .and_then(consume_close_paren);

        remaining = match parse_result {
            Ok(((multiplicand, multiplier), new_remaining)) => {
                result += multiplicand * multiplier;
                new_remaining
            }
            Err(new_remaining) => new_remaining,
        };
    }

    Ok(result)
}

#[derive(Debug)]
enum ProgramError {
    Parse(String),
    IO(std::io::Error),
}
fn main() -> Result<(), ProgramError> {
    let f = File::open("day-03-input.txt").map_err(ProgramError::IO)?;
    let reader = BufReader::new(f);
    let mut result = 0;
    for maybe_line in reader.lines() {
        let line = maybe_line.map_err(ProgramError::IO)?;
        result += sum_multiplications(&line).map_err(ProgramError::Parse)?;
    }

    println!("{}", result);

    Ok(())
}
