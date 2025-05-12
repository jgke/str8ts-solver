use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::Grid;

pub fn trivial(grid: &mut Grid) -> bool {
    let mut changes = false;
    for (pos, cell) in grid.iter_by_cells() {
        let set = cell.to_unresolved();
        if set.len() == 1 {
            grid.set_cell(pos, Solution(set.into_iter().next().unwrap()));
            changes = true;
        }
    }

    let missing_numbers: BitSet = (1..=grid.x as u8).collect();
    if grid.has_requirements() {
        for (vertical, row) in grid.iter_by_rows_and_cols() {
            let sample_pos = row[0].0;

            let mut missing_numbers = missing_numbers;
            for (_, cell) in row {
                match cell {
                    Indeterminate(set) => {
                        missing_numbers = missing_numbers.difference(set);
                    }
                    Requirement(n) | Solution(n) => {
                        missing_numbers.remove(n);
                        changes |= grid.requirements_mut(vertical, sample_pos).insert(n);
                    }
                    Blocker(n) => {
                        changes |= grid.forbidden_mut(vertical, sample_pos).insert(n);
                    }
                    Black => {}
                }
            }
            grid.forbidden_mut(vertical, sample_pos)
                .append(missing_numbers);
        }
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Cell;
    use crate::utils::*;

    #[test]
    fn trivial_updates() {
        let mut grid = g("
#.#
..#
#..
");
        grid.cells[0][1] = det([1]);
        grid.cells[1][1] = det([1, 2]);
        grid.cells[1][0] = det([2]);
        grid.cells[2][2] = det([3]);

        assert!(trivial(&mut grid));

        assert_eq!(grid.cells[0][0], Cell::Black);
        assert_eq!(grid.cells[0][1], Cell::Solution(1));
        assert_eq!(grid.cells[0][2], Cell::Black);

        assert_eq!(grid.cells[1][0], Cell::Solution(2));
        assert_eq!(grid.cells[1][1], det([1, 2]));
    }

    #[test]
    fn required_forbidden_updates() {
        let mut grid = g("
#.b
..#
#..
");
        grid.cells[0][1] = det([1]);
        grid.cells[1][1] = det([1, 2]);
        grid.cells[1][0] = det([2]);
        grid.cells[2][2] = det([3]);

        assert!(trivial(&mut grid));
        grid.row_requirements[1].insert(1);
        assert!(trivial(&mut grid));

        assert_eq!(grid.row_requirements[0], set([1]));
        assert_eq!(grid.row_requirements[1], set([1, 2]));
        assert_eq!(grid.row_requirements[2], set([3]));

        assert_eq!(grid.col_requirements[0], set([2]));
        assert_eq!(grid.col_requirements[1], set([1]));
        assert_eq!(grid.col_requirements[2], set([3]));

        assert_eq!(grid.row_forbidden[0], set([2, 3]));
        assert_eq!(grid.row_forbidden[1], set([3]));
        assert_eq!(grid.row_forbidden[2], set([]));

        assert_eq!(grid.col_forbidden[0], set([1, 3]));
        assert_eq!(grid.col_forbidden[1], set([]));
        assert_eq!(grid.col_forbidden[2], set([1, 2]));
    }
}
