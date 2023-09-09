use rustc_hash::FxHashSet;
use std::fmt::{Display, Formatter};

use crate::bitset::BitSet;
use crate::solver::ValidationResult;
use Cell::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Requirement(u8),
    Solution(u8),
    Blocker(u8),
    Indeterminate(BitSet),
    Black,
}

pub type CellPair = ((usize, usize), Cell);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Compartment {
    pub cells: Vec<CellPair>,
    pub vertical: bool,
}

impl Compartment {
    pub fn contains(&self, num: u8) -> bool {
        self.cells.iter().any(|(_, cell)| match cell {
            Requirement(n) | Solution(n) => *n == num,
            Indeterminate(set) => set.contains(num),
            Blocker(_) | Black => false,
        })
    }
}

impl Cell {
    pub fn to_req_or_sol(&self) -> Option<u8> {
        match self {
            Requirement(c) | Solution(c) => Some(*c),
            Blocker(_) | Indeterminate(_) | Black => None,
        }
    }

    pub fn to_determinate(&self) -> Option<u8> {
        match self {
            Requirement(c) | Solution(c) | Blocker(c) => Some(*c),
            Indeterminate(_) | Black => None,
        }
    }

    pub fn to_possibles(&self) -> BitSet {
        match self {
            Requirement(c) | Solution(c) => [*c].into_iter().collect(),
            Indeterminate(set) => *set,
            Blocker(_) | Black => BitSet::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
    pub x: usize,
    pub y: usize,
    pub row_requirements: Vec<BitSet>,
    pub col_requirements: Vec<BitSet>,
    pub row_forbidden: Vec<BitSet>,
    pub col_forbidden: Vec<BitSet>,
}

impl Grid {
    pub fn new(cells: Vec<Vec<Cell>>) -> Result<Grid, String> {
        let size = cells.len();
        if size == 0 || cells.iter().any(|row| row.len() != size) {
            return Err("Invalid grid dimensions".to_string());
        }
        Ok(Grid {
            cells,
            x: size,
            y: size,
            row_requirements: (0..size).map(|_| BitSet::default()).collect(),
            col_requirements: (0..size).map(|_| BitSet::default()).collect(),
            row_forbidden: (0..size).map(|_| BitSet::default()).collect(),
            col_forbidden: (0..size).map(|_| BitSet::default()).collect(),
        })
    }

    pub fn get_cell(&self, pos: (usize, usize)) -> &Cell {
        &self.cells[pos.1][pos.0]
    }

    pub fn set_cell(&mut self, pos: (usize, usize), cell: Cell) {
        let (x, y) = pos;
        self.cells[y][x] = cell;
    }

    pub fn is_solved(&self) -> bool {
        self.cells.iter().flat_map(|c| c.iter()).all(|c| match c {
            Requirement(_) | Solution(_) | Blocker(_) | Black => true,
            Indeterminate(_) => false,
        })
    }

    pub fn get_row(&self, y: usize) -> Vec<CellPair> {
        self.cells[y]
            .iter()
            .enumerate()
            .map(move |(x, cell)| ((x, y), cell.clone()))
            .collect()
    }

    pub fn iter_by_rows(&self) -> Vec<Vec<CellPair>> {
        self.cells
            .iter()
            .enumerate()
            .map(|(y, c)| {
                c.iter()
                    .enumerate()
                    .map(move |(x, cell)| ((x, y), cell.clone()))
                    .collect()
            })
            .collect()
    }

    pub fn get_col(&self, x: usize) -> Vec<CellPair> {
        self.cells
            .iter()
            .enumerate()
            .map(move |(y, row)| ((x, y), row[x].clone()))
            .collect()
    }
    pub fn iter_by_cols(&self) -> Vec<Vec<CellPair>> {
        (0..self.x)
            .map(|x| {
                self.cells
                    .iter()
                    .enumerate()
                    .map(move |(y, c)| ((x, y), c[x].clone()))
                    .collect()
            })
            .collect()
    }

    pub fn iter_by_rows_and_cols(&self) -> Vec<(bool, Vec<CellPair>)> {
        self.iter_by_rows()
            .into_iter()
            .map(|row| (false, row))
            .chain(self.iter_by_cols().into_iter().map(|row| (true, row)))
            .collect()
    }

    pub fn iter_by_cells(&self) -> Vec<CellPair> {
        self.iter_by_rows()
            .into_iter()
            .flat_map(|row| row.into_iter())
            .collect()
    }

    pub fn iter_by_cell_pos_matching<F>(&self, mut predicate: F) -> Vec<(usize, usize)>
    where
        F: FnMut(&Cell) -> bool,
    {
        self.iter_by_rows()
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter_map(|((x, y), cell)| if predicate(&cell) { Some((x, y)) } else { None })
            .collect()
    }

    pub fn line_to_compartments(vertical: bool, line: Vec<CellPair>) -> Vec<Compartment> {
        let mut containers = Vec::new();
        let mut cells = Vec::new();
        for (pos, cell) in line {
            match cell {
                Requirement(_) | Solution(_) | Indeterminate(_) => cells.push((pos, cell)),
                Black | Blocker(_) => {
                    if !cells.is_empty() {
                        containers.push(Compartment { cells, vertical });
                        cells = Vec::new();
                    }
                }
            }
        }
        if !cells.is_empty() {
            containers.push(Compartment { cells, vertical });
        }

        containers
    }

    pub fn iter_by_row_compartments(&self) -> Vec<Vec<Compartment>> {
        self.iter_by_rows()
            .into_iter()
            .map(|row| Self::line_to_compartments(false, row))
            .collect()
    }

    pub fn iter_by_col_compartments(&self) -> Vec<Vec<Compartment>> {
        self.iter_by_cols()
            .into_iter()
            .map(|row| Self::line_to_compartments(true, row))
            .collect()
    }

    pub fn iter_by_compartments(&self) -> Vec<Vec<Compartment>> {
        self.iter_by_row_compartments()
            .into_iter()
            .chain(self.iter_by_col_compartments())
            .collect()
    }

    pub fn set_impossible(
        &mut self,
        pos: (usize, usize),
        num: u8,
    ) -> Result<bool, ValidationResult> {
        let (x, y) = pos;
        if let Cell::Indeterminate(ref mut set) = self.cells[y][x] {
            let ret = set.remove(num);
            if set.is_empty() {
                return Err(ValidationResult::EmptyCell { pos });
            }
            return Ok(ret);
        }
        Ok(false)
    }

    pub fn remove_numbers(
        &mut self,
        pos: (usize, usize),
        nums: BitSet,
    ) -> Result<bool, ValidationResult> {
        let mut retval = false;
        for num in nums {
            retval |= self.set_impossible(pos, num)?;
        }
        Ok(retval)
    }

    pub fn set_impossible_in(
        &mut self,
        sample_pos: (usize, usize),
        vertical: bool,
        impossible: u8,
        except_in: &FxHashSet<(usize, usize)>,
    ) -> Result<bool, ValidationResult> {
        let mut changes = false;
        if !vertical {
            let y = sample_pos.1;
            for x in 0..self.x {
                if !except_in.contains(&(x, y)) {
                    changes |= self.set_impossible((x, y), impossible)?;
                }
            }
        } else {
            let x = sample_pos.0;
            for y in 0..self.y {
                if !except_in.contains(&(x, y)) {
                    changes |= self.set_impossible((x, y), impossible)?;
                }
            }
        }
        Ok(changes)
    }

    pub fn has_requirements(&self) -> bool {
        self.row_requirements
            .iter()
            .chain(self.col_requirements.iter())
            .chain(self.row_forbidden.iter())
            .chain(self.col_forbidden.iter())
            .any(|s| !s.is_empty())
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
                .take_while(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();

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
        } else {
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
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;

        for row in &self.cells {
            if !first {
                writeln!(f)?;
            }
            first = false;
            for cell in row {
                match cell {
                    Requirement(n) => write!(f, "{}", n)?,
                    Solution(n) => write!(f, "{}", n)?,
                    Blocker(n) => write!(f, "{}", (n - 1 + b'a') as char)?,
                    Indeterminate(_) => write!(f, ".")?,
                    Black => write!(f, "#")?,
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Cell;
    use crate::utils::*;

    #[test]
    fn test_set_impossible() {
        let mut grid = g("
###
###
###
");
        grid.set_cell((0, 0), Cell::Black);
        grid.set_cell((1, 0), Cell::Requirement(1));
        grid.set_cell((2, 0), Cell::Solution(1));
        grid.set_cell((0, 1), Cell::Blocker(1));
        grid.set_cell((1, 1), Cell::Indeterminate(set([1, 2])));
        grid.set_cell((2, 1), Cell::Indeterminate(set([1, 2])));

        for y in 0..=1 {
            for x in 0..=2 {
                assert_eq!(grid.set_impossible((x, y), 1), Ok(x >= 1 && y == 1));
            }
        }

        assert_eq!(grid.get_cell((0, 0)), &Cell::Black);
        assert_eq!(grid.get_cell((1, 0)), &Cell::Requirement(1));
        assert_eq!(grid.get_cell((2, 0)), &Cell::Solution(1));
        assert_eq!(grid.get_cell((0, 1)), &Cell::Blocker(1));
        assert_eq!(grid.get_cell((1, 1)), &Cell::Indeterminate(set([2])));
        assert_eq!(grid.get_cell((2, 1)), &Cell::Indeterminate(set([2])));
    }

    #[test]
    fn test_remove_numbers() {
        let mut grid = g("
###
###
###
");
        grid.set_cell((0, 0), Cell::Black);
        grid.set_cell((1, 0), Cell::Requirement(1));
        grid.set_cell((2, 0), Cell::Solution(1));
        grid.set_cell((0, 1), Cell::Blocker(1));
        grid.set_cell((1, 1), Cell::Indeterminate(set([1, 2, 3])));
        grid.set_cell((2, 1), Cell::Indeterminate(set([1, 2, 4])));

        let remove_set: BitSet = [1, 2].into_iter().collect();
        for y in 0..=1 {
            for x in 0..=2 {
                assert_eq!(
                    grid.remove_numbers((x, y), remove_set),
                    Ok(x >= 1 && y == 1)
                );
            }
        }

        assert_eq!(grid.get_cell((0, 0)), &Cell::Black);
        assert_eq!(grid.get_cell((1, 0)), &Cell::Requirement(1));
        assert_eq!(grid.get_cell((2, 0)), &Cell::Solution(1));
        assert_eq!(grid.get_cell((0, 1)), &Cell::Blocker(1));
        assert_eq!(grid.get_cell((1, 1)), &Cell::Indeterminate(set([3])));
        assert_eq!(grid.get_cell((2, 1)), &Cell::Indeterminate(set([4])));
    }
}
