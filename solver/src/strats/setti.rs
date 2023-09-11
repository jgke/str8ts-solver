use crate::bitset::BitSet;
use crate::grid::Grid;

pub fn setti(grid: &mut Grid) -> Option<BitSet> {
    let mut changes = BitSet::new();

    for n in 1..=grid.x as u8 {
        let mut row_min = 0;
        let mut row_max = grid.x;
        for y in 0..grid.x {
            if grid.row_requirements[y].contains(n) {
                row_min += 1;
            }
            if grid.row_forbidden[y].contains(n) {
                row_max -= 1;
            }
        }

        let mut col_min = 0;
        let mut col_max = grid.x;
        for x in 0..grid.x {
            if grid.col_requirements[x].contains(n) {
                col_min += 1;
            }
            if grid.col_forbidden[x].contains(n) {
                col_max -= 1;
            }
        }

        let res: Vec<_> = ((row_min.max(col_min))..=(row_max.min(col_max))).collect();

        if res.len() == 1 {
            let setti_count = res[0];
            let mut local_changes = false;

            if row_max == setti_count {
                for y in 0..grid.x {
                    if !grid.row_forbidden[y].contains(n) {
                        local_changes |= grid.row_requirements[y].insert(n);
                    }
                }
            } else if row_min == setti_count {
                for y in 0..grid.x {
                    if !grid.row_requirements[y].contains(n) {
                        local_changes |= grid.row_forbidden[y].insert(n);
                    }
                }
            }

            if col_max == setti_count {
                for x in 0..grid.x {
                    if !grid.col_forbidden[x].contains(n) {
                        local_changes |= grid.col_requirements[x].insert(n);
                    }
                }
            } else if col_min == setti_count {
                for x in 0..grid.x {
                    if !grid.col_requirements[x].contains(n) {
                        local_changes |= grid.col_forbidden[x].insert(n);
                    }
                }
            }

            if local_changes {
                changes.insert(n);
            }
        }
    }

    if changes.is_empty() {
        None
    } else {
        Some(changes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::solve_basic;
    use crate::solver::SolveResults::OutOfBasicStrats;
    use crate::strats::update_required_and_forbidden;
    use crate::utils::*;

    #[test]
    fn test_setti() {
        let mut grid = g("
.1...
.....
.....
.....
.#...
");

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(grid.row_forbidden[4], set([]));

        assert_eq!(setti(&mut grid), Some(set([1, 2, 4, 5])));

        assert_eq!(grid.row_forbidden[4], set([5]));

        assert_eq!(grid.cells[4][0], det([1, 2, 4, 5]));
        assert_eq!(grid.cells[4][2], det([1, 2, 3, 4, 5]));
        assert_eq!(grid.cells[4][3], det([1, 2, 3, 4, 5]));
        assert_eq!(grid.cells[4][4], det([1, 2, 3, 4, 5]));

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(grid.cells[4][0], det([1, 4]));
        assert_eq!(grid.cells[4][2], det([1, 2, 3, 4]));
        assert_eq!(grid.cells[4][3], det([1, 2, 3, 4]));
        assert_eq!(grid.cells[4][4], det([1, 2, 3, 4]));
    }
}
