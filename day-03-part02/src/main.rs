use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn consume_remainder(input: &str) -> &str {
    let (_, no_remainder) = input.split_at(input.len());
    no_remainder
}

enum Instruction {
    Mul,
    Do,
    Dont,
}
impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value {
            "mul" => Instruction::Mul,
            "do()" => Instruction::Do,
            "don't()" => Instruction::Dont,
            _ => unreachable!(),
        }
    }
}

fn scan_for_next_instruction(input: &str) -> Result<(Instruction, &str), &str> {
    let instructions = ["mul", "do()", "don't()"];
    let mut remaining = input;
    while input.len() > 0 {
        for i in instructions {
            if remaining.starts_with(i) {
                let (_, new_remaining) = remaining.split_at(i.len());
                return Ok((i.into(), new_remaining));
            }
        }

        if remaining.len() == 0 {
            return Err("");
        }

        let (_, new_remaining) = remaining.split_at(1);
        remaining = new_remaining;
    }

    Err("")
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

fn handle_mult_instruction(input: &str) -> Result<(i32, &str), &str> {
    consume_open_paren(input)
        .and_then(read_digits)
        .and_then(consume_comma)
        .and_then(read_second_number)
        .and_then(consume_close_paren)
        .and_then(|((multiplicand, multiplier), new_remaining)| {
            Ok((multiplicand * multiplier, new_remaining))
        })
}

fn sum_multiplications<'a>(mut enabled: bool, input: &'a str) -> Result<(bool, i32), String> {
    let mut remaining = input;
    let mut result = 0;
    while remaining.len() > 0 {
        let (add_this_to_sum, new_remaining) = scan_for_next_instruction(&remaining)
            .and_then(|(ins, r)| match ins {
                Instruction::Mul => handle_mult_instruction(r),
                Instruction::Do => {
                    enabled = true;
                    Ok((0, r))
                }
                Instruction::Dont => {
                    enabled = false;
                    Ok((0, r))
                }
            })
            .unwrap_or_else(|r| (0, r));

        if enabled {
            result += add_this_to_sum;
        }
        remaining = new_remaining;
    }

    Ok((enabled, result))
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
    let mut enabled = true;
    for maybe_line in reader.lines() {
        let line = maybe_line.map_err(ProgramError::IO)?;
        let (new_enabled, line_result) =
            sum_multiplications(enabled, &line).map_err(ProgramError::Parse)?;
        result += line_result;
        enabled = new_enabled;
    }

    println!("{}", result);

    Ok(())
}
