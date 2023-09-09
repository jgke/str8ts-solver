use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::Grid;
use crate::solver::ValidationResult;

pub fn stranded(grid: &mut Grid) -> Result<bool, ValidationResult> {
    let mut changes = false;

    for row in grid.iter_by_compartments() {
        for compartment in row {
            let compartment_size = compartment.cells.len();
            for ((x, y), cell) in &compartment.cells {
                if let Indeterminate(set) = cell {
                    let nums: BitSet = compartment
                        .cells
                        .iter()
                        .filter(|(pos, _)| pos != &(*x, *y))
                        .flat_map(|(_, c)| c.to_possibles().into_iter())
                        .collect();
                    let mut to_remove = BitSet::default();
                    for start_num in *set {
                        let mut min = start_num;
                        let mut max = start_num;
                        while min > 1 {
                            if nums.contains(min - 1) {
                                min -= 1;
                            } else {
                                break;
                            }
                        }

                        while max <= grid.x as u8 {
                            if nums.contains(max + 1) {
                                max += 1;
                            } else {
                                break;
                            }
                        }

                        if max - min + 1 < compartment_size as u8 {
                            to_remove.insert(start_num);
                        }
                    }
                    changes |= grid.remove_numbers((*x, *y), to_remove)?;
                }
            }
        }
    }

    Ok(changes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_stranded() {
        let mut grid = g("
####
#..#
#.##
####
");
        grid.cells[1][1] = det([1, 2]);
        grid.cells[2][1] = det([1, 2, 4]);
        grid.cells[1][2] = det([1, 2, 4]);
        assert_eq!(stranded(&mut grid), Ok(true));
        assert_eq!(grid.cells[1][1], det([1, 2]));
        assert_eq!(grid.cells[2][1], det([1, 2]));
        assert_eq!(grid.cells[1][2], det([1, 2]));
    }
}
