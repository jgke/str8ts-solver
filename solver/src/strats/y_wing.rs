use crate::grid::Grid;
use crate::solve_result::SolveType::YWing;
use crate::solve_result::{SolveMetadata, SolveResults, ValidationResult};

#[allow(clippy::type_complexity)]
pub fn y_wing(grid: &mut Grid) -> Result<Option<SolveResults>, ValidationResult> {
    let valid_indeterminates = grid
        .iter_by_indeterminates()
        .into_iter()
        .filter(|(_, set)| set.len() == 2)
        .collect::<Vec<_>>();

    for &((x, y), ab) in &valid_indeterminates {
        let row_matches = valid_indeterminates
            .iter()
            .filter(|&&((xx, yy), _)| y == yy && x != xx)
            .collect::<Vec<_>>();

        let col_matches = valid_indeterminates
            .iter()
            .filter(|&&((xx, yy), _)| x == xx && y != yy)
            .collect::<Vec<_>>();

        for &&(pos1, other_set_1) in &row_matches {
            let abc = ab.union(other_set_1);
            if abc.len() != 3 {
                continue;
            }

            for &&(pos2, other_set_2) in &col_matches {
                if ab.union(other_set_2) != abc {
                    continue;
                }
                if other_set_1 == other_set_2 {
                    continue;
                }
                let c = abc.difference(ab);
                assert_eq!(c.len(), 1);
                let num = c.into_iter().next().unwrap();
                let pos = (pos1.0, pos2.1);

                if grid.set_impossible(pos, num)? {
                    let colors = vec![abc
                        .into_iter()
                        .flat_map(|num| vec![((x, y), num), (pos1, num), (pos2, num)])
                        .collect()];
                    let meta = SolveMetadata { colors };
                    let ty = YWing(pos, num);
                    return Ok(Some(SolveResults { meta, ty }));
                }
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve_result::ValidationError::OutOfStrats;
    use crate::solver::solve_basic;
    use crate::utils::*;

    #[test]
    fn test_wing() {
        let mut grid = g("
.....
.....
..##.
..#..
.....
");

        grid.cells[1][1] = det([1, 2]);
        grid.cells[3][1] = det([2, 3]);
        grid.cells[1][3] = det([1, 3]);
        grid.cells[3][3] = det([1, 2, 3]);

        assert_eq!(solve_basic(&mut grid), Err(OutOfStrats));

        assert_eq!(grid.cells[3][3], det([1, 2, 3]));

        assert_eq!(
            Ok(Some(SolveResults {
                ty: YWing((3, 3), 3),
                meta: SolveMetadata {
                    colors: vec![vec![
                        ((1, 1), 1),
                        ((3, 1), 1),
                        ((1, 3), 1),
                        ((1, 1), 2),
                        ((3, 1), 2),
                        ((1, 3), 2),
                        ((1, 1), 3),
                        ((3, 1), 3),
                        ((1, 3), 3)
                    ]]
                }
            })),
            y_wing(&mut grid)
        );

        assert_eq!(grid.cells[3][3], det([1, 2]));
    }
}
