use crate::bitset::BitSet;
use crate::grid::Grid;
use crate::solve_result::SolveType::UpdateImpossibles;
use crate::strategy::StrategyReturn;

pub fn update_impossibles(grid: &mut Grid) -> StrategyReturn {
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
        for ((x, y), _) in row {
            changes |= grid.remove_numbers((x, y), row_impossibles[y])?;
            changes |= grid.remove_numbers((x, y), col_impossibles[x])?;
        }
    }

    if changes {
        Ok(Some(UpdateImpossibles.into()))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Cell;
    use crate::utils::*;
    #[test]
    fn test_update_impossibles() {
        let mut grid = g("
####
#4.#
#.##
####
");
        assert_eq!(update_impossibles(&mut grid), Ok(Some(UpdateImpossibles.into())));
        assert_eq!(grid.cells[1][1], Cell::Requirement(4));
        assert_eq!(grid.cells[2][1], det([1, 2, 3]));
        assert_eq!(grid.cells[1][2], det([1, 2, 3]));
    }
}
