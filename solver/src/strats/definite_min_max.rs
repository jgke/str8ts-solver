use crate::bitset::BitSet;
use crate::grid::Compartment;
use crate::grid::Grid;
use crate::solver::ValidationResult;
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
            .map(|((x, y), _)| ((x, y), grid.get_cell((x, y)).clone()))
            .collect(),
        vertical: stale_compartment.vertical,
    }
}

pub fn set_range(set: BitSet, min: u8, max: u8) -> BitSet {
    set.into_iter().filter(|c| *c >= min && *c <= max).collect()
}

pub fn definite_min_max(grid: &mut Grid) -> Result<bool, ValidationResult> {
    let mut changes = false;

    for compartment in grid.iter_by_compartments() {
        if let Some((min, max)) = get_compartment_range(grid.x, &compartment, None) {
            for ((x, y), cell) in compartment.cells {
                let set = cell.to_unresolved();
                if !set.is_empty() {
                    let new_set = set_range(set, min, max);
                    changes |= grid.remove_numbers((x, y), set.difference(new_set))?;
                }
            }
        }
    }

    if grid.has_requirements() {
        for compartment in grid.iter_by_compartments() {
            let pos = compartment.sample_pos();
            let reqs = grid.requirements(compartment.vertical, pos);
            for n in reqs {
                let compartment = update_data(grid, compartment.clone());
                if compartment.contains(n) && num_count_in_containers(grid, &compartment, n) == 1 {
                    if let Some((min, max)) = get_compartment_range(grid.x, &compartment, Some(n)) {
                        for ((x, y), cell) in &compartment.cells {
                            let set = cell.to_unresolved();
                            if !set.is_empty() {
                                let new_set = set_range(set, min, max);
                                changes |=
                                    grid.remove_numbers((*x, *y), set.difference(new_set))?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(changes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Cell;
    use crate::solver::solve_basic;
    use crate::solver::SolveType::OutOfBasicStrats;
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
        assert_eq!(strats::update_impossibles(&mut grid), Ok(true));
        assert_eq!(definite_min_max(&mut grid), Ok(true));
        assert_eq!(grid.cells[1][1], Cell::Requirement(4));
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

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(strats::update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(strats::setti(&mut grid), Some(set([5])));

        assert_eq!(grid.cells[3][2], det([3, 4, 5]));
        assert_eq!(grid.cells[4][2], det([3, 4, 5]));

        assert_eq!(definite_min_max(&mut grid), Ok(true));

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

        assert_eq!(definite_min_max(&mut grid), Ok(true));

        assert_eq!(grid.cells[1][0], det([1, 2]));
        assert_eq!(grid.cells[1][1], det([1, 2]));
        assert_eq!(grid.cells[1][3], det([6]));
        assert_eq!(grid.cells[1][4], det([6, 7]));
        assert_eq!(grid.cells[1][5], det([5]));
        assert_eq!(grid.cells[1][7], det([3, 4]));
        assert_eq!(grid.cells[1][8], det([3, 4]));
    }
}
