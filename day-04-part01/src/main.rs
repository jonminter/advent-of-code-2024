use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() > 1);

    let input_file = &args[1];

    let puzzle = build_puzzle_map_from_file(input_file)
        .map_err(|e| format!("Error building puzzle map: {}", e))?;

    let mut word_coords = Vec::new();

    let xmas_count: usize = get_all_coordinates(&puzzle)
        .filter(|(x, y)| letter_at(&puzzle, *x, *y) == 'X')
        .map(|(x, y)| {
            let (num_found, coords) = search_from_coord(&puzzle, x, y);

            word_coords.extend(coords);
            num_found
        })
        .sum();
    println!("{} xmas", xmas_count);

    draw_board(&puzzle, &word_coords.into_iter().collect());

    Ok(())
}

fn get_all_coordinates(puzzle: &Vec<Vec<char>>) -> impl Iterator<Item = (usize, usize)> + '_ {
    puzzle
        .iter()
        .enumerate()
        .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, _)| (y, x)))
}

fn build_puzzle_map_from_file(file_path: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    reader
        .lines()
        .map(|maybe_l| maybe_l.map(|l| l.chars().collect()))
        .collect()
}

static NEXT_CHAR: &[(char, char)] = &[('A', 'S'), ('M', 'A'), ('X', 'M')];

pub fn get_next_char(c: char) -> Option<char> {
    NEXT_CHAR
        .binary_search_by(|(k, _)| k.cmp(&c))
        .map(|x| NEXT_CHAR[x].1)
        .ok()
}

fn search_from_coord(puzzle: &Vec<Vec<char>>, x: usize, y: usize) -> (usize, Vec<(usize, usize)>) {
    let paths: [[(i32, i32); 3]; 8] = [
        [(-1, -1), (-2, -2), (-3, -3)], // UL
        [(0, -1), (0, -2), (0, -3)],    //U
        [(1, -1), (2, -2), (3, -3)],    // UR
        [(-1, 0), (-2, 0), (-3, 0)],    // L
        [(1, 0), (2, 0), (3, 0)],       // R
        [(-1, 1), (-2, 2), (-3, 3)],    //DL
        [(0, 1), (0, 2), (0, 3)],       // D
        [(1, 1), (2, 2), (3, 3)],       // DR
    ];

    let mut word_coords: Vec<(usize, usize)> = Vec::new();

    let num_found_xmas = paths
        .iter()
        .map(|p| get_next_three_coords(puzzle, x, y, p))
        .filter(|p| p.is_ok())
        .filter(|p| spells_xmas(puzzle, p.unwrap()))
        .map(|p| {
            word_coords.push((x, y));
            word_coords.extend_from_slice(p.unwrap().as_slice());

            1
        })
        .sum();

    (num_found_xmas, word_coords)
}

fn get_next_three_coords<'a>(
    puzzle: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    path: &'a [(i32, i32); 3],
) -> Result<[(usize, usize); 3], ()> {
    Ok([
        new_position(puzzle, x, y, path[0].0, path[0].1).ok_or(())?,
        new_position(puzzle, x, y, path[1].0, path[1].1).ok_or(())?,
        new_position(puzzle, x, y, path[2].0, path[2].1).ok_or(())?,
    ])
}

fn new_position(
    puzzle: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    move_x: i32,
    move_y: i32,
) -> Option<(usize, usize)> {
    let x: i32 = x.try_into().unwrap();
    let y: i32 = y.try_into().unwrap();
    let new_x: i32 = x + move_x;
    let new_y: i32 = y + move_y;
    let puzzle_w: i32 = puzzle[0].len().try_into().unwrap();
    let puzzle_h: i32 = puzzle.len().try_into().unwrap();

    if new_x < 0 || new_x >= puzzle_w || new_y < 0 || new_y >= puzzle_h {
        return None;
    }

    Some((new_x.try_into().unwrap(), new_y.try_into().unwrap()))
}

fn spells_xmas(puzzle: &Vec<Vec<char>>, p: [(usize, usize); 3]) -> bool {
    let maybe_m = letter_at(puzzle, p[0].0, p[0].1);
    let maybe_a = letter_at(puzzle, p[1].0, p[1].1);
    let maybe_s = letter_at(puzzle, p[2].0, p[2].1);

    maybe_m == 'M' && maybe_a == 'A' && maybe_s == 'S'
}

fn is_valid_coord(puzzle: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let in_bounds = y < puzzle.len() && x < puzzle[0].len();

    if !in_bounds {
        println!("!in_bounds {:?}", (x, y));
    }
    in_bounds
}

fn letter_at(puzzle: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    assert!(is_valid_coord(puzzle, x, y));

    puzzle[y][x]
}

const COLOR_GREEN: &str = "\x1b[92m";
const COLOR_RED: &str = "\x1b[91m";
const COLOR_YELLOW: &str = "\x1b[93m";
const COLOR_RESET: &str = "\x1b[0m";

fn draw_board(puzzle: &Vec<Vec<char>>, highlight: &HashSet<(usize, usize)>) {
    let mut f: Vec<(usize, usize)> = get_all_coordinates(puzzle).collect();
    f.sort();

    let h = puzzle.len();
    let w = puzzle[0].len();

    let mut next = (0, 0);

    println!("{}", (1..(w * 2 + 2)).map(|_| '-').collect::<String>());
    loop {
        print!("|");

        let should_highlight = highlight.contains(&(next.0, next.1));
        let c = letter_at(puzzle, next.0, next.1);
        if should_highlight {
            print!(
                "{}",
                match c {
                    'X' => COLOR_GREEN,
                    'S' => COLOR_RED,
                    _ => COLOR_YELLOW,
                }
            );
        }

        print!("{}", c);

        if should_highlight {
            print!("{}", COLOR_RESET)
        }

        next = (next.0 + 1, next.1);

        if next.0 >= w {
            print!("|");
            println!("");
            println!("{}", (1..(w * 2 + 2)).map(|_| '-').collect::<String>());
            next = (0, next.1 + 1);
        }

        if next.1 >= h {
            break;
        }
    }
}
