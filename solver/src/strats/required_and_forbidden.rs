use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::{CellPair, Compartment, Grid};

pub fn required_in_compartment_by_range(grid_size: usize, compartment: &Compartment) -> BitSet {
    let mut required = BitSet::default();
    let compartment_size = compartment.cells.len();
    if let Some((min, max)) = get_compartment_range(grid_size, compartment, None) {
        let def_max = min - 1 + (compartment_size as u8);
        let def_min = max + 1 - (compartment_size as u8);

        for num in def_min..=def_max {
            required.insert(num);
        }
    }
    required
}

pub fn required_by_range(grid_size: usize, line: &[CellPair]) -> BitSet {
    Grid::line_to_compartments(false, line.to_vec())
        .into_iter()
        .flat_map(|compartment| {
            required_in_compartment_by_range(grid_size, &compartment).into_iter()
        })
        .collect()
}

pub fn required_by_certain(line: &[CellPair]) -> BitSet {
    let mut required = BitSet::default();

    for compartment in Grid::line_to_compartments(false, line.to_vec()) {
        for (_, cell) in compartment.cells {
            match cell {
                Requirement(n) => {
                    required.insert(n);
                }
                Solution(n) => {
                    required.insert(n);
                }
                Blocker(_) => {}
                Indeterminate(_) => {}
                Black => {}
            }
        }
    }

    required
}

pub fn required_numbers(grid: &Grid, line: &[CellPair]) -> BitSet {
    required_by_certain(line)
        .into_iter()
        .chain(required_by_range(grid.x, line))
        .collect()
}

pub fn forbidden_by_certain(line: &[CellPair]) -> BitSet {
    let mut required = BitSet::default();

    for (_, cell) in line {
        if let Blocker(n) = cell {
            required.insert(*n);
        }
    }

    required
}

pub fn forbidden_numbers(_grid: &Grid, line: &[CellPair]) -> BitSet {
    forbidden_by_certain(line)
}

pub fn possible_numbers(line: &[CellPair]) -> BitSet {
    line.iter()
        .flat_map(|(_, cell)| {
            let iter: Box<dyn Iterator<Item = _>> = match cell {
                Requirement(n) | Solution(n) => Box::new([*n].into_iter()),
                Blocker(_) => Box::new(std::iter::empty()),
                Indeterminate(set) => Box::new(set.into_iter()),
                Black => Box::new(std::iter::empty()),
            };
            iter
        })
        .collect()
}

pub fn get_compartment_range(
    grid_size: usize,
    compartment: &Compartment,
    must_contain: Option<u8>,
) -> Option<(u8, u8)> {
    let len = compartment.cells.len();

    let ranges: Vec<(u8, u8)> = compartment
        .cells
        .iter()
        .filter_map(|(_, cell)| match (cell.to_determinate(), cell) {
            (Some(n), _) => Some((n, n)),
            (_, Indeterminate(set)) => Some((
                set.into_iter().min().unwrap(),
                set.into_iter().max().unwrap(),
            )),
            (_, _) => None,
        })
        .chain(must_contain.into_iter().map(|n| (n, n)))
        .collect();

    let absolute_smallest = ranges.iter().map(|(min, _)| *min).min()?;
    let absolute_biggest = ranges.iter().map(|(_, max)| *max).max()?;

    let biggest_left = ranges.iter().map(|(min, _)| *min).max()?;
    let smallest_right = ranges.iter().map(|(_, max)| *max).min()?;

    let min = biggest_left.saturating_sub(len as u8 - 1).max(1);
    let max = (smallest_right + len as u8 - 1).min(grid_size as u8);

    Some((min.max(absolute_smallest), max.min(absolute_biggest)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_compartment_range() {
        assert_eq!(
            Some((1, 1)),
            get_compartment_range(
                1,
                &Compartment {
                    cells: vec![((0, 0), Requirement(1))],
                    vertical: false
                },
                None
            )
        );
        assert_eq!(
            Some((1, 2)),
            get_compartment_range(
                2,
                &Compartment {
                    cells: vec![((0, 0), Requirement(1)), ((0, 0), Requirement(2)),],
                    vertical: false
                },
                None
            )
        );
        assert_eq!(
            Some((1, 2)),
            get_compartment_range(
                3,
                &Compartment {
                    cells: vec![((0, 0), Requirement(1)), ((0, 0), Requirement(2)),],
                    vertical: false
                },
                None
            )
        );
        assert_eq!(
            Some((2, 3)),
            get_compartment_range(
                4,
                &Compartment {
                    cells: vec![((0, 0), Requirement(2)), ((0, 0), Requirement(3)),],
                    vertical: false
                },
                None
            )
        );
        assert_eq!(
            Some((3, 4)),
            get_compartment_range(
                4,
                &Compartment {
                    cells: vec![((0, 0), Requirement(3)), ((0, 0), Requirement(4)),],
                    vertical: false
                },
                None
            )
        );
    }

    #[test]
    fn test_indeterminate_compartment_range() {
        assert_eq!(
            Some((1, 1)),
            get_compartment_range(
                1,
                &Compartment {
                    cells: vec![((0, 0), det([1]))],
                    vertical: false
                },
                None
            )
        );
    }

    #[test]
    fn test_forced_compartment_range() {
        assert_eq!(
            Some((1, 4)),
            get_compartment_range(
                5,
                &Compartment {
                    cells: vec![
                        ((0, 0), det([1, 2, 3, 4, 5])),
                        ((0, 0), det([1, 2, 3, 4, 5])),
                        ((0, 0), det([1, 2, 3, 4, 5])),
                    ],
                    vertical: false
                },
                Some(2)
            )
        );
    }
}
