//! Puzzle encoding / decoding, based on the str8ts.com string definition format
//! https://www.str8ts.com/Str8ts_String_Definitions

use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::{Cell, Grid};
use crate::utils::format_radix;

fn parse_format_1(row: Vec<char>) -> Result<Grid, String> {
    let size = ((row.len() / 2) as f64).sqrt() as usize;
    let size2 = size * size;
    if 2 * size2 != row.len() {
        return Err("Did not recognize puzzle format: Tried to detect as oneline but dimensions did not match".to_string());
    }

    let mut cells = Vec::new();
    for y in 0..size {
        let mut res = Vec::new();
        for x in 0..size {
            match (row[y * size + x], row[y * size + x + size2]) {
                ('0', '0') => res.push(Indeterminate((1..=size as u8).collect())),
                ('0', '1') => res.push(Black),
                (c @ '1'..='9', '0') => res.push(Requirement((c as u8) - b'0')),
                (c @ '1'..='9', '1') => res.push(Blocker((c as u8) - b'0')),
                (other, _) => return Err(format!("Unexpected character: {}", other)),
            }
        }
        cells.push(res);
    }

    Grid::new(cells)
}

fn parse_format_2(puzzle: Vec<char>) -> Result<Grid, String> {
    if puzzle.len() < 3 {
        return Err("Could not parse puzzle".to_string());
    }

    let _ty = match puzzle[0] {
        'T' => "normal",
        'U' => return Err("Str8ts X puzzles not supported".to_string()),
        'B' => return Err("Str8ts B puzzles not supported".to_string()),
        'X' => return Err("Str8ts BX puzzles not supported".to_string()),
        _ => return Err(format!("Unknown puzzle type '{}'", puzzle[0])),
    };

    let size = if let Some(digit) = puzzle[1].to_digit(36) {
        digit
    } else {
        return Err(format!("Unknown puzzle size '{}'", puzzle[1]));
    } as usize;

    let _version = match puzzle[2] {
        'B' => 2,
        _ => return Err(format!("Unknown puzzle version '{}'", puzzle[2])),
    };

    if puzzle.len() != 2 * size * size + 3 {
        return Err(format!(
            "Invalid puzzle string length; expected {} but got {}",
            2 * size * size + 3,
            puzzle.len()
        ));
    }

    let mut cells = Vec::new();

    for y in 0..size {
        let mut row = Vec::new();
        for x in 0..size {
            let n_str = &puzzle[2 * (y * size + x) + 3..=2 * (y * size + x) + 3 + 1];
            let n = 36 * n_str[0].to_digit(36).unwrap_or(0) + n_str[1].to_digit(36).unwrap_or(0);
            let cell: Cell = if n < 10 {
                Requirement(n as u8)
            } else if n == 10 {
                Black
            } else if n < 20 {
                Blocker((n - 10) as u8)
            } else if n < 29 {
                Solution((n - 20) as u8)
            } else {
                Indeterminate(BitSet::new_from_number((n - 29) << 1))
            };
            row.push(cell);
        }
        cells.push(row);
    }

    Grid::new(cells)
}

fn parse_format_grid(puzzle: Vec<String>) -> Result<Grid, String> {
    let mut cells = Vec::new();
    let size = puzzle.len();
    for row in puzzle {
        let mut res = Vec::new();
        for c in row.chars() {
            match c {
                '1'..='9' => res.push(Requirement((c as u8) - b'0')),
                'a'..='i' => res.push(Blocker((c as u8) - b'a' + 1)),
                '.' => res.push(Indeterminate((1..=size as u8).collect())),
                '#' => res.push(Black),
                other => return Err(format!("Unexpected character: {}", other)),
            }
        }
        cells.push(res);
    }

    Grid::new(cells)
}

pub fn parse(puzzle: Vec<String>) -> Result<Grid, String> {
    let puzzle = puzzle
        .join("\n")
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    if puzzle.len() == 1 {
        let row = puzzle[0]
            .split("bd=")
            .last()
            .unwrap()
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric())
            .collect::<Vec<_>>();

        if ['T', 'U', 'B', 'X'].contains(row.first().unwrap_or(&'.')) {
            parse_format_2(row)
        } else {
            parse_format_1(row)
        }
    } else {
        parse_format_grid(puzzle)
    }
}

pub fn encode(grid: &Grid) -> String {
    let format = 'T'.to_string();
    let size = match grid.x {
        1..9 => grid.x.to_string(),
        16 => 'G'.to_string(),
        25 => 'P'.to_string(),
        _ => '9'.to_string(),
    };
    let version = 'B';

    let mut cells = String::new();
    for y in 0..grid.y {
        for x in 0..grid.x {
            let num = match *grid.get_cell((x, y)) {
                Requirement(num) => num as u32,
                Solution(num) => (num as u32) + 20,
                Blocker(num) => (num as u32) + 10,
                Indeterminate(set) => (set.to_number() >> 1) + 29,
                Black => 10,
            };
            let s = format_radix(num, 36);
            if s.chars().count() == 1 {
                cells.push('0');
            }
            cells.push_str(&format_radix(num, 36))
        }
    }

    format!("{}{}{}{}", format, size, version, cells)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{det, g};

    #[test]
    fn test_parse_url() {
        let url = "https://jgke.fi/games/str8ts-solver?bd=T5B03021a0e1a0a010a0a030p0403020l0b1d1j1d1j131d1k1e1k";
        assert_eq!(
            Grid::new(vec![
                vec![
                    Requirement(3),
                    Requirement(2),
                    det([1, 5]),
                    Blocker(4),
                    det([1, 5])
                ],
                vec![Black, Requirement(1), Black, Black, Requirement(3)],
                vec![
                    Solution(5),
                    Requirement(4),
                    Requirement(3),
                    Requirement(2),
                    Solution(1)
                ],
                vec![
                    Blocker(1),
                    det([3, 5]),
                    det([2, 4, 5]),
                    det([3, 5]),
                    det([2, 4, 5])
                ],
                vec![
                    det([2, 4]),
                    det([3, 5]),
                    det([1, 2, 4, 5]),
                    det([1, 3, 5]),
                    det([1, 2, 4, 5])
                ]
            ]),
            parse(vec![url.to_string()])
        );
    }

    #[test]
    fn test_encode() {
        let mut grid = g("
#.c
..#
#.2
");
        grid.cells[0][1] = det([1]);
        grid.cells[1][0] = det([1, 2]);
        let _ = crate::strats::trivial(&mut grid);

        assert_eq!("T3B0a0l0d0w100a0a1002", encode(&grid));
    }
}
