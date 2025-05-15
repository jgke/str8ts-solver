use crate::bitset::BitSet;
use crate::grid::{Grid, Point};
use crate::solve_result::{SolveMetadata, SolveResults, SolveType, ValidationResult};
use itertools::Itertools;
use rustc_hash::FxHashSet;

pub fn sets(grid: &mut Grid) -> Result<Option<SolveResults>, ValidationResult> {
    let mut changes = false;
    let mut colored_sets: Vec<(Vec<Point>, BitSet)> = Vec::new();

    for n in 2..grid.x {
        for (vertical, line) in grid.iter_by_rows_and_cols() {
            let sets: Vec<(Point, BitSet)> = line
                .into_iter()
                .map(|(p, c)| (p, c.to_unresolved()))
                .filter(|(_, c)| !c.is_empty())
                .collect();
            let used_nums: BitSet = sets.iter().flat_map(|(_, set)| set.into_iter()).collect();

            if used_nums.len() <= 2 || sets.len() <= 2 {
                continue;
            }

            for try_set_vec in used_nums.into_iter().combinations(n) {
                let try_set: BitSet = try_set_vec.into_iter().collect();
                let mut applies_to = FxHashSet::default();

                for (pos, set) in &sets {
                    if set.union(try_set) == try_set {
                        applies_to.insert(*pos);
                    }
                }

                if applies_to.len() == n {
                    let mut local_changes = false;
                    for n in try_set {
                        local_changes |= grid.set_impossible_in(
                            applies_to.iter().copied().next().unwrap(),
                            vertical,
                            n,
                            &applies_to,
                        )?;
                    }

                    if grid.has_requirements() {
                        let sample_pos = applies_to.iter().copied().next().unwrap();
                        changes |= grid.requirements_mut(vertical, sample_pos).append(try_set);
                    }
                    changes |= local_changes;
                    if local_changes {
                        colored_sets.push((applies_to.into_iter().sorted().collect(), try_set));
                        break;
                    }
                }
            }
        }

        if changes {
            let colors = colored_sets
                .into_iter()
                .map(|(points, set)| {
                    set.into_iter()
                        .flat_map(|n| points.iter().copied().map(move |point| (point, n)))
                        .collect()
                })
                .collect();
            let meta = SolveMetadata { colors };
            let ty = SolveType::Sets(n);
            return Ok(Some(SolveResults { ty, meta }));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve_result::SolveType::Sets;
    use crate::utils::*;

    #[test]
    fn test_sets() {
        let mut grid = g("
########
#..##..#
#.####.#
#.######
########
#.####.#
#..##..#
########
");
        grid.cells[1][1] = det([1, 2]);
        grid.cells[1][2] = det([1, 2]);
        grid.cells[2][1] = det([1, 2, 3]);
        grid.cells[3][1] = det([1, 2, 3]);

        /* round 1: n=2 */
        assert_eq!(
            Ok(Some(SolveResults {
                ty: Sets(2),
                meta: SolveMetadata {
                    colors: vec![vec![((1, 1), 1), ((2, 1), 1), ((1, 1), 2), ((2, 1), 2)]]
                }
            })),
            sets(&mut grid)
        );

        assert_eq!(grid.cells[1][1], det([1, 2]));
        assert_eq!(grid.cells[1][2], det([1, 2]));
        assert_eq!(grid.cells[2][1], det([1, 2, 3]));
        assert_eq!(grid.cells[3][1], det([1, 2, 3]));

        assert_eq!(grid.cells[1][5], det([3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[1][6], det([3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[2][6], det([1, 2, 3, 4, 5, 6, 7, 8]));

        assert_eq!(grid.cells[5][1], det([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[6][1], det([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[6][2], det([1, 2, 3, 4, 5, 6, 7, 8]));

        assert_eq!(grid.cells[6][5], det([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[5][6], det([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[6][6], det([1, 2, 3, 4, 5, 6, 7, 8]));

        /* round 2: n=3 */
        assert_eq!(
            Ok(Some(SolveResults {
                ty: Sets(3),
                meta: SolveMetadata {
                    colors: vec![vec![
                        ((1, 1), 1),
                        ((1, 2), 1),
                        ((1, 3), 1),
                        ((1, 1), 2),
                        ((1, 2), 2),
                        ((1, 3), 2),
                        ((1, 1), 3),
                        ((1, 2), 3),
                        ((1, 3), 3)
                    ]]
                }
            })),
            sets(&mut grid)
        );

        assert_eq!(grid.cells[1][1], det([1, 2]));
        assert_eq!(grid.cells[1][2], det([1, 2]));
        assert_eq!(grid.cells[2][1], det([1, 2, 3]));
        assert_eq!(grid.cells[3][1], det([1, 2, 3]));

        assert_eq!(grid.cells[1][5], det([3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[1][6], det([3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[2][6], det([1, 2, 3, 4, 5, 6, 7, 8]));

        assert_eq!(grid.cells[5][1], det([4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[6][1], det([4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[6][2], det([1, 2, 3, 4, 5, 6, 7, 8]));

        assert_eq!(grid.cells[6][5], det([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[5][6], det([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[6][6], det([1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    fn test_sets_requirements() {
        let mut grid = g("
########
#..##..#
#.####.#
#.######
########
#.####.#
#..##..#
########
");
        grid.cells[1][1] = det([1, 2]);
        grid.cells[1][2] = det([1, 2]);
        grid.cells[2][1] = det([1, 2, 3]);
        grid.cells[3][1] = det([1, 2, 3]);
        grid.row_requirements[0].insert(1);

        /* round 1: n=2 */
        assert_eq!(
            Ok(Some(SolveResults {
                ty: Sets(2),
                meta: SolveMetadata {
                    colors: vec![vec![((1, 1), 1), ((2, 1), 1), ((1, 1), 2), ((2, 1), 2)]]
                }
            })),
            sets(&mut grid)
        );

        assert_eq!(grid.row_requirements[1], set([1, 2]));
        assert_eq!(grid.col_requirements[1], set([]));

        /* round 2: n=3 */
        assert_eq!(
            Ok(Some(SolveResults {
                ty: Sets(3),
                meta: SolveMetadata {
                    colors: vec![vec![
                        ((1, 1), 1),
                        ((1, 2), 1),
                        ((1, 3), 1),
                        ((1, 1), 2),
                        ((1, 2), 2),
                        ((1, 3), 2),
                        ((1, 1), 3),
                        ((1, 2), 3),
                        ((1, 3), 3)
                    ]]
                }
            })),
            sets(&mut grid)
        );

        assert_eq!(grid.row_requirements[1], set([1, 2]));
        assert_eq!(grid.col_requirements[1], set([1, 2, 3]));
    }
}
