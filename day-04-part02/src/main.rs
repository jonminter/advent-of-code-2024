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
        .filter(|(x, y)| letter_at(&puzzle, *x, *y) == 'A')
        .filter(|(x, y)| {
            search_from_coord(&puzzle, *x, *y, &mut word_coords)
        })
        .map(|_| 1)
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


fn search_from_coord(puzzle: &Vec<Vec<char>>, x: usize, y: usize, coords_in_xmas: &mut Vec<(usize, usize)>) -> bool {
    let paths: [[(i32, i32); 2]; 2] = [
        [(-1, -1), (1, 1)],  // UL, BR
        [(-1, 1), (1, -1)], // UR, BL
        
    ];

    let bounds_checked_paths = [get_next_two_coords(puzzle, x,y,&paths[0]), get_next_two_coords(puzzle, x, y, &paths[1])];
    if bounds_checked_paths.iter().any(|p| p.is_err()) {
        return false;
    }
    let bounds_checked_paths = [bounds_checked_paths[0].unwrap(), bounds_checked_paths[1].unwrap()];

    if bounds_checked_paths.iter().any(|p| !spells_mas(puzzle, *p)) {
        return false;
    }

    coords_in_xmas.push((x,y));
    coords_in_xmas.push(bounds_checked_paths[0][0]);
    coords_in_xmas.push(bounds_checked_paths[0][1]);
    coords_in_xmas.push(bounds_checked_paths[1][0]);
    coords_in_xmas.push(bounds_checked_paths[1][1]);

    true
}

fn get_next_two_coords<'a>(
    puzzle: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    path: &'a [(i32, i32); 2],
) -> Result<[(usize, usize); 2], ()> {
    Ok([
        new_position(puzzle, x, y, path[0].0, path[0].1).ok_or(())?,
        new_position(puzzle, x, y, path[1].0, path[1].1).ok_or(())?,
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

fn spells_mas(puzzle: &Vec<Vec<char>>, path: [(usize, usize); 2]) -> bool {
    let letter = [letter_at(puzzle, path[0].0, path[0].1), letter_at(puzzle, path[1].0, path[1].1)];
    letter == ['M', 'S'] || letter == ['S', 'M']
}

fn is_valid_coord(puzzle: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    y < puzzle.len() && x < puzzle[0].len()
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

    println!(" {}", (0..w).map(|i| format!(" {}", i)).collect::<String>());
    println!(" {}", (1..(w * 2 + 2)).map(|_| '-').collect::<String>());

    print!("0");
    let mut curr_y = 0;
    loop {
        print!("|");

        let should_highlight = highlight.contains(&(next.0, next.1));
        let c = letter_at(puzzle, next.0, next.1);
        if should_highlight {
            print!(
                "{}",
                match c {
                    'M' => COLOR_GREEN,
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
            curr_y += 1;
            print!("|");
            println!("");
            println!(" {}", (1..(w * 2 + 2)).map(|_| '-').collect::<String>());
            if curr_y < w {
                print!("{}", curr_y);
            }
            next = (0, next.1 + 1);
        }

        if next.1 >= h {
            break;
        }
    }
}
