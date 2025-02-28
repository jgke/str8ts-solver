use crate::grid::{CellPair, Grid};
use crate::solver::ValidationResult;
use rustc_hash::FxHashSet;

fn get_cells_with_indeterminate_num(line: &[CellPair], num: u8) -> Vec<(usize, usize)> {
    let cells: Vec<(usize, usize)> = line
        .iter()
        .filter_map(|(pos, cell)| if cell.to_unresolved().contains(num) {
            Some(*pos)
        } else {
            None
        })
        .collect();
    cells
}

pub fn fish(grid: &mut Grid) -> Result<Option<usize>, ValidationResult> {
    let mut changes = false;

    fn same_lane(vertical: bool, a: (usize, usize), b: (usize, usize)) -> bool {
        if vertical {
            a.0 == b.0
        } else {
            a.1 == b.1
        }
    }

    for fish_count in 2..grid.x {
        for (vertical, lines, reqs) in [
            (false, grid.iter_by_rows(), grid.row_requirements.clone()),
            (true, grid.iter_by_cols(), grid.col_requirements.clone()),
        ] {
            for (idx, line) in lines.clone().into_iter().enumerate() {
                for num in reqs[idx] {
                    let cells = get_cells_with_indeterminate_num(&line, num);

                    if cells.len() != fish_count {
                        continue;
                    }

                    let mut candidates = Vec::new();
                    for other_line in &lines {
                        let other_cells = get_cells_with_indeterminate_num(other_line, num);
                        if other_cells.len() != fish_count {
                            continue;
                        }

                        let other_idx = if !vertical {
                            assert_eq!(other_cells[0].1, other_cells[1].1);
                            other_cells[0].1
                        } else {
                            assert_eq!(other_cells[0].0, other_cells[1].0);
                            other_cells[0].0
                        };

                        if !reqs[other_idx].contains(num) {
                            continue;
                        }

                        if other_cells
                            .iter()
                            .zip(cells.iter())
                            .all(|(left, right)| same_lane(!vertical, *left, *right))
                        {
                            candidates.push(other_cells);
                        }
                    }

                    if candidates.len() > fish_count {
                        // puzzle not solvable
                        let positions: FxHashSet<(usize, usize)> = candidates
                            .iter()
                            .flat_map(|line| line.iter())
                            .copied()
                            .collect();
                        for position in &positions {
                            changes |= grid.set_impossible(*position, num)?;
                        }
                        changes = true;
                    }
                    if candidates.len() == fish_count {
                        let positions: FxHashSet<(usize, usize)> = candidates
                            .iter()
                            .flat_map(|line| line.iter())
                            .copied()
                            .collect();
                        for position in &positions {
                            changes |= grid.set_impossible_in(*position, false, num, &positions)?;
                            changes |= grid.set_impossible_in(*position, true, num, &positions)?;
                        }

                        for (x, y) in positions {
                            grid.row_requirements[y].insert(num);
                            grid.col_requirements[x].insert(num);
                        }
                    }
                }
            }
        }
        if changes {
            return Ok(Some(fish_count));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::solve_basic;
    use crate::solver::SolveResults::OutOfBasicStrats;
    use crate::strats::{setti, update_required_and_forbidden};
    use crate::utils::*;

    #[test]
    fn test_wing() {
        let mut grid = g("
.....
.....
.....
##..1
##..2
");

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(setti(&mut grid), Some(set([2])));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(grid.cells[0][2], det([1, 2, 3, 4, 5]));
        assert_eq!(grid.cells[0][3], det([1, 2, 3, 4, 5]));
        assert_eq!(grid.cells[1][2], det([1, 2, 3, 4, 5]));
        assert_eq!(grid.cells[1][3], det([1, 2, 3, 4, 5]));
        assert_eq!(grid.cells[2][2], det([1, 2, 3, 4, 5]));
        assert_eq!(grid.cells[2][3], det([1, 2, 3, 4, 5]));

        assert_eq!(Ok(Some(2)), fish(&mut grid));

        assert_eq!(grid.cells[0][2], det([1, 2, 4, 5]));
        assert_eq!(grid.cells[0][3], det([1, 2, 4, 5]));
        assert_eq!(grid.cells[1][2], det([1, 2, 4, 5]));
        assert_eq!(grid.cells[1][3], det([1, 2, 4, 5]));
        assert_eq!(grid.cells[2][2], det([1, 2, 4, 5]));
        assert_eq!(grid.cells[2][3], det([1, 2, 4, 5]));
    }
}
