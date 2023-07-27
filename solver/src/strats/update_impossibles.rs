use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::Grid;

pub fn update_impossibles(grid: &mut Grid) -> bool {
    let mut row_impossibles: Vec<BitSet> = Vec::new();
    for (y, row) in grid.iter_by_rows().into_iter().enumerate() {
        row_impossibles.push(
            row.into_iter()
                .filter_map(|(_, c)| c.to_determinate())
                .chain(grid.row_forbidden[y].into_iter())
                .collect(),
        );
    }
    let mut col_impossibles: Vec<BitSet> = Vec::new();
    for (x, col) in grid.iter_by_cols().into_iter().enumerate() {
        col_impossibles.push(
            col.into_iter()
                .filter_map(|(_, c)| c.to_determinate())
                .chain(grid.col_forbidden[x].into_iter())
                .collect(),
        )
    }

    let mut changes = false;
    for row in grid.iter_by_rows().into_iter() {
        for ((x, y), cell) in row {
            match cell {
                Indeterminate(set) => {
                    let new_set: BitSet = set.difference(row_impossibles[y]);
                    if set.len() != new_set.len() {
                        grid.cells[y][x] = Indeterminate(new_set);
                        changes = true;
                    }
                }
                Requirement(_) | Solution(_) | Blocker(_) | Black => {}
            }
        }
    }

    for col in grid.iter_by_cols().into_iter() {
        for ((x, y), cell) in col {
            match cell {
                Indeterminate(set) => {
                    let new_set: BitSet = set.difference(col_impossibles[x]);
                    if set.len() != new_set.len() {
                        grid.cells[y][x] = Indeterminate(new_set);
                        changes = true;
                    }
                }
                Requirement(_) | Solution(_) | Blocker(_) | Black => {}
            }
        }
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test_update_impossibles() {
        let mut grid = g("
####
#4.#
#.##
####
");
        assert!(update_impossibles(&mut grid));
        assert_eq!(grid.cells[1][1], Requirement(4));
        assert_eq!(grid.cells[2][1], det([1, 2, 3]));
        assert_eq!(grid.cells[1][2], det([1, 2, 3]));
    }
}
