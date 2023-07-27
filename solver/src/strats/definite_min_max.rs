use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::Grid;
use crate::strats::get_compartment_range;

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

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
