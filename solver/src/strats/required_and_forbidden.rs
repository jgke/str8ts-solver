use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::{CellPair, Compartment, Grid};

pub fn required_in_compartment_by_range(grid_size: usize, compartment: &Compartment) -> BitSet {
    let free_numbers = compartment.combined_unresolved();
    if compartment.to_unresolved().len() == free_numbers.len() {
        return free_numbers;
    }

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
        .flat_map(|compartment| required_in_compartment_by_range(grid_size, &compartment).into_iter())
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
            (_, Indeterminate(set)) => Some((set.into_iter().min().unwrap(), set.into_iter().max().unwrap())),
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
