use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::Compartment;
use crate::grid::Grid;
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

pub fn update_data(grid: &Grid, stale_compartment: Compartment) -> Compartment {
    Compartment {
        cells: stale_compartment
            .cells
            .into_iter()
            .map(|((x, y), _)| ((x, y), grid.cells[y][x].clone()))
            .collect(),
        vertical: stale_compartment.vertical,
    }
}

pub fn definite_min_max(grid: &mut Grid) -> bool {
    let mut changes = false;

    for row in grid.iter_by_compartments() {
        for compartment in row {
            if let Some((min, max)) = get_compartment_range(grid.x, &compartment, BitSet::default())
            {
                for ((x, y), cell) in compartment.cells {
                    match cell {
                        Indeterminate(set) => {
                            let new_set: BitSet =
                                set.into_iter().filter(|c| *c >= min && *c <= max).collect();
                            if set.len() != new_set.len() {
                                grid.cells[y][x] = Indeterminate(new_set);
                                changes = true;
                            }
                        }
                        Requirement(_) | Solution(_) | Blocker(_) | Black => {}
                    }
                }
            }
        }
    }

    if grid.has_requirements() {
        for line in grid.iter_by_compartments().into_iter() {
            for compartment in line {
                let (pos, _) = compartment.cells[0];
                let reqs = if compartment.vertical {
                    grid.col_requirements[pos.0]
                } else {
                    grid.row_requirements[pos.1]
                };
                for n in reqs {
                    let compartment = update_data(grid, compartment.clone());
                    if compartment.contains(n)
                        && num_count_in_containers(grid, &compartment, n) == 1
                    {
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
                                            let new_val = Indeterminate(new_set);
                                            grid.cells[*y][*x] = new_val;
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
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::run_basic;
    use crate::solver::SolveResults::OutOfBasicStrats;
    use crate::strats;
    use crate::utils::*;

    #[test]
    fn test_definite_range() {
        let mut grid = g("
####
#4.#
#.##
####
");
        assert!(strats::update_impossibles(&mut grid));
        assert!(definite_min_max(&mut grid));
        assert_eq!(grid.cells[1][1], Requirement(4));
        assert_eq!(grid.cells[2][1], det([3]));
        assert_eq!(grid.cells[1][2], det([3]));
    }

    #[test]
    fn test_setti_min_max() {
        let mut grid = g("
..1..
5#...
.##.5
.....
.....
");

        while run_basic(&mut grid) != OutOfBasicStrats {}

        assert!(strats::update_required_and_forbidden(&mut grid));
        assert_eq!(strats::setti(&mut grid), Some(set([5])));

        assert_eq!(grid.cells[3][2], det([3, 4, 5]));
        assert_eq!(grid.cells[4][2], det([3, 4, 5]));

        assert!(definite_min_max(&mut grid));

        assert_eq!(grid.cells[3][2], det([4, 5]));
        assert_eq!(grid.cells[4][2], det([4, 5]));
    }

    #[test]
    fn min_max_regression() {
        let mut grid = g("
#########
#########
#########
#########
#########
#########
#########
#########
#########
");

        grid.cells[1][0] = det([1, 2, 3, 4, 5, 6, 7]);
        grid.cells[1][1] = det([1, 2, 3, 4, 5, 6, 7]);
        grid.cells[1][3] = det([4, 6]);
        grid.cells[1][4] = det([3, 6, 7]);
        grid.cells[1][5] = det([2, 5]);
        grid.cells[1][7] = det([3, 4, 5]);
        grid.cells[1][8] = det([3, 4, 5, 6]);

        grid.row_requirements[1].append(set([1, 2, 3, 4, 5, 6, 7]));

        assert!(definite_min_max(&mut grid));

        assert_eq!(grid.cells[1][0], det([1, 2]));
        assert_eq!(grid.cells[1][1], det([1, 2]));
        assert_eq!(grid.cells[1][3], det([6]));
        assert_eq!(grid.cells[1][4], det([6, 7]));
        assert_eq!(grid.cells[1][5], det([5]));
        assert_eq!(grid.cells[1][7], det([3, 4]));
        assert_eq!(grid.cells[1][8], det([3, 4]));
    }
}
