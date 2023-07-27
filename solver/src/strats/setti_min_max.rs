use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::{Compartment, Grid};
use crate::strats::get_compartment_range;

pub fn num_count_in_containers(grid: &Grid, current_compartment: &Compartment, num: u8) -> usize {
    let (x, y) = current_compartment.cells[0].0;
    let line = if current_compartment.vertical {
        grid.get_col(x)
    } else {
        grid.get_row(y)
    };
    Grid::line_to_compartments(current_compartment.vertical, line)
        .into_iter()
        .map(|compartment| compartment.contains(num))
        .filter(|x| *x)
        .count()
}

pub fn setti_min_max(grid: &mut Grid) -> bool {
    let mut changes = false;

    for line in grid.iter_by_compartments().into_iter() {
        for compartment in line {
            let (pos, _) = compartment.cells[0];
            let reqs = if compartment.vertical {
                grid.col_requirements[pos.0]
            } else {
                grid.row_requirements[pos.1]
            };
            for n in reqs {
                if compartment.contains(n) && num_count_in_containers(grid, &compartment, n) == 1 {
                    if let Some((min, max)) =
                        get_compartment_range(grid.x, &compartment, [n].into_iter().collect())
                    {
                        for ((x, y), cell) in &compartment.cells {
                            match cell {
                                Indeterminate(set) => {
                                    let new_set: BitSet = set
                                        .into_iter()
                                        .filter(|c| *c >= min && *c <= max)
                                        .collect();
                                    if set.len() != new_set.len() {
                                        grid.cells[*y][*x] = Indeterminate(new_set);
                                        changes = true;
                                    }
                                }
                                Requirement(_) | Solution(_) | Blocker(_) | Black => {}
                            }
                        }
                    }
                }
            }
        }
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::run_basic;
    use crate::solver::SolveResults::OutOfBasicStrats;
    use crate::strats::{setti, update_required_and_forbidden};
    use crate::utils::*;

    #[test]
    fn test_setti() {
        let mut grid = g("
..1..
5#...
.##.5
.....
.....
");

        while run_basic(&mut grid) != OutOfBasicStrats {}

        update_required_and_forbidden(&mut grid);

        assert!(setti(&mut grid));

        assert_eq!(grid.cells[3][2], det([3, 4, 5]));
        assert_eq!(grid.cells[4][2], det([3, 4, 5]));

        assert!(setti_min_max(&mut grid));

        assert_eq!(grid.cells[3][2], det([4, 5]));
        assert_eq!(grid.cells[4][2], det([4, 5]));
    }
}
