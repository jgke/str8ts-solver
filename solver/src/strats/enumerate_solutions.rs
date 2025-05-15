use crate::bitset::BitSet;
use crate::grid::Cell::Solution;
use crate::grid::{Grid, Point};
use crate::solve_result::{SolveMetadata, SolveResults, SolveType, ValidationError, ValidationResult};
use crate::validator::validate;

fn iter(grid: &Grid, indeterminates: &[(Point, BitSet)]) -> Vec<Grid> {
    if let Some((&(pos, set), rest)) = indeterminates.split_first() {
        let mut res = Vec::new();
        for num in set {
            let mut inner = grid.clone();
            inner.set_cell(pos, Solution(num));
            if validate(grid).is_ok() {
                res.extend(iter(&inner, rest));
            }
        }
        res
    } else if validate(grid).is_ok() {
        vec![grid.clone()]
    } else {
        vec![]
    }
}

const MAX_INDETERMINATES: usize = 8;
pub fn enumerate_solutions(grid: &mut Grid) -> Result<Option<SolveResults>, ValidationResult> {
    let indeterminates = grid.iter_by_indeterminates();
    if indeterminates.len() > MAX_INDETERMINATES {
        return Ok(None);
    }

    let solutions = iter(grid, &indeterminates);
    if solutions.is_empty() {
        /* Should be unreachable */
        Err(ValidationResult {
            ty: ValidationError::NoSolutions,
            meta: SolveMetadata { colors: vec![] },
        })
    } else if solutions.len() == 1 {
        /* Should be unreachable */
        for (pos, cell) in solutions[0].iter_by_cells() {
            grid.set_cell(pos, cell);
        }
        Ok(Some(SolveResults {
            ty: SolveType::EnumerateSolutions,
            meta: SolveMetadata { colors: vec![] },
        }))
    } else {
        let colors = solutions
            .into_iter()
            .map(|grid| {
                indeterminates
                    .iter()
                    .map(|&(pos, _)| (pos, grid.get_cell(pos).to_determinate().unwrap()))
                    .collect()
            })
            .collect();
        Err(ValidationResult {
            ty: ValidationError::Ambiguous {
                cells: indeterminates.iter().map(|(pos, _)| *pos).collect(),
            },
            meta: SolveMetadata { colors },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_no_solutions() {
        let mut grid = g("
12
35
");

        assert_eq!(
            enumerate_solutions(&mut grid),
            Err(ValidationResult {
                ty: ValidationError::NoSolutions,
                meta: SolveMetadata { colors: vec![] }
            })
        );
    }

    #[test]
    fn test_one_solution() {
        let mut grid = g("
1.
23
");

        assert_eq!(
            enumerate_solutions(&mut grid),
            Ok(Some(SolveResults {
                ty: SolveType::EnumerateSolutions,
                meta: SolveMetadata { colors: vec![] }
            }))
        );
        assert_eq!(grid.cells[0][1], Solution(2));
    }

    #[test]
    fn test_multiple_solutions() {
        let mut grid = g("
..
..
");

        assert_eq!(
            enumerate_solutions(&mut grid),
            Err(ValidationResult {
                ty: ValidationError::Ambiguous {
                    cells: vec![(0, 0), (1, 0), (0, 1), (1, 1)]
                },
                meta: SolveMetadata {
                    colors: vec![
                        vec![((0, 0), 1), ((1, 0), 2), ((0, 1), 2), ((1, 1), 1)],
                        vec![((0, 0), 2), ((1, 0), 1), ((0, 1), 1), ((1, 1), 2)]
                    ]
                }
            })
        );
    }

    #[test]
    fn test_over_limit() {
        let mut grid = g("
.....
.....
.....
.....
.....
");

        assert_eq!(enumerate_solutions(&mut grid), Ok(None));
    }
}
