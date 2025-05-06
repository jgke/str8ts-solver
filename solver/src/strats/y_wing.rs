use crate::grid::Grid;
use crate::solver::ValidationResult;

#[allow(clippy::type_complexity)]
pub fn y_wing(grid: &mut Grid) -> Result<Option<((usize, usize), u8)>, ValidationResult> {
    let valid_indeterminates = grid
        .iter_by_indeterminates()
        .into_iter()
        .filter(|(_, set)| set.len() == 2)
        .collect::<Vec<_>>();

    for ((x, y), ab) in &valid_indeterminates {
        let row_matches = valid_indeterminates
            .iter()
            .filter(|((xx, yy), _)| y == yy && x != xx)
            .collect::<Vec<_>>();

        let col_matches = valid_indeterminates
            .iter()
            .filter(|((xx, yy), _)| x == xx && y != yy)
            .collect::<Vec<_>>();

        for (pos1, other_set_1) in &row_matches {
            let abc = ab.union(*other_set_1);
            if abc.len() != 3 {
                continue;
            }

            for (pos2, other_set_2) in &col_matches {
                if ab.union(*other_set_2) != abc {
                    continue;
                }
                let c = abc.difference(*ab);
                assert_eq!(c.len(), 1);
                let num = c.into_iter().next().unwrap();
                let pos = (pos1.0, pos2.1);

                if grid.set_impossible(pos, num)? {
                    return Ok(Some((pos, num)));
                }
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::solve_basic;
    use crate::solver::SolveResults::OutOfBasicStrats;
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

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(grid.cells[3][3], det([1, 2, 3]));

        assert_eq!(Ok(Some(((3, 3), 3))), y_wing(&mut grid));

        assert_eq!(grid.cells[3][3], det([1, 2]));
    }
}
