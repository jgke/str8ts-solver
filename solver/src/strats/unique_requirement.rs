use crate::bitset::BitSet;
use crate::grid::{Cell, Compartment, Grid, Point};
use crate::solver::ValidationResult;
use crate::strats::get_compartment_range;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UrResult {
    SingleUnique(Point, u8),
    IntraCompartmentUnique(Point, u8),
    ClosedSetCompartment(Vec<Point>, u8),
    SingleCellWouldBecomeFree(Point, u8),
    UrSetti(Vec<Point>, bool, u8),
    SolutionCausesClosedSets(Point, u8),
}

/* If a cell is implied only by other compartments, and those compartments don't refer to a
 * candidate the cell has, the cell has to be that candidate as otherwise it would cause the puzzle
 * to be ambiguous, eg.:
 * 15 pointed by [123] [123] [123] and [123] [123] [123]
 * -> cell has to be 5, as after putting in 1 it could be always replaced by 5, but not vice versa
 */
fn single_cell_cross_compartment_unique(
    grid: &mut Grid,
    x: usize,
    y: usize,
    set: BitSet,
) -> Result<Option<UrResult>, ValidationResult> {
    let mut free_set = set;
    let (row, col) = grid.compartments_containing((x, y));
    if row.to_unresolved().len() == 1 && col.to_unresolved().len() == 1 {
        for (p, c) in grid.get_row(y) {
            if p != (x, y) {
                free_set = free_set.difference(c.to_possibles());
            }
        }
        for (p, c) in grid.get_col(x) {
            if p != (x, y) {
                free_set = free_set.difference(c.to_possibles());
            }
        }

        if free_set.len() > 1 {
            return Err(ValidationResult::Ambiguous {
                cells: vec![(x, y)],
            });
        }
        if let Some(res) = free_set.into_iter().next() {
            grid.set_cell((x, y), Cell::Solution(res));
            return Ok(Some(UrResult::SingleUnique((x, y), res)));
        }
    }
    Ok(None)
}

/* If a cell contains both ends of a possible compartment range, and other compartments imply only
 * one end of the range, it cannot be that number, as otherwise it would cause ambiguity, eg.:
 *
 * [23]
 * [1234] [1234] # [456] [456]
 * [23]
 * #
 * [456]
 * [456]
 * [456]
 *
 * The [1234] cell at (0, 1) cannot be 4, as otherwise it could be always replaced by 1 without
 * affecting other cells. It might or might not be 1.
 */
fn single_cell_intra_compartment_unique(
    grid: &mut Grid,
    x: usize,
    y: usize,
    set: BitSet,
) -> Result<Option<UrResult>, ValidationResult> {
    fn closed_set(compartment: &Compartment) -> bool {
        compartment.to_unresolved().len() == compartment.combined_unresolved().len()
    }

    let mut free_set = set;
    let (row, col) = grid.compartments_containing((x, y));
    let (minx, maxx) = get_compartment_range(grid.x, &row, None).unwrap();
    let (miny, maxy) = get_compartment_range(grid.y, &col, None).unwrap();
    let range_size = maxx - minx + 1;

    if minx != miny || maxx != maxy {
        return Ok(None);
    }
    let row_len = row.cells.len() as u8;
    let col_len = col.cells.len() as u8;
    #[allow(clippy::nonminimal_bool)]
    if (row_len + 1 == range_size || col_len + 1 == range_size)
        && (row_len == range_size || row_len + 1 == range_size)
        && (col_len == range_size || col_len + 1 == range_size)
        && free_set.contains(minx)
        && free_set.contains(maxx)
    {
        if closed_set(&row) || closed_set(&col) {
            return Ok(None);
        }
        for (p, c) in grid.get_row(y) {
            if !row.contains_pos(p) {
                free_set = free_set.difference(c.to_possibles());
            }
        }
        for (p, c) in grid.get_col(x) {
            if !col.contains_pos(p) {
                free_set = free_set.difference(c.to_possibles());
            }
        }

        if free_set.contains(minx) {
            grid.set_impossible((x, y), maxx)?;
            return Ok(Some(UrResult::IntraCompartmentUnique((x, y), maxx)));
        }
        if free_set.contains(maxx) {
            grid.set_impossible((x, y), minx)?;
            return Ok(Some(UrResult::IntraCompartmentUnique((x, y), minx)));
        }
    }
    Ok(None)
}

/* If a compartment has a cell with no other implicators than the rest of the compartment, it has
 * to be uniquely identifiable based on the other cells:
 *
 * [123]
 * [123] [123]
 *
 * The [123] at (0, 1) cannot be 2, as the cell in (0, 0) could be either 1 or 3.
 */
fn single_cell_would_become_free(
    grid: &mut Grid,
    x: usize,
    y: usize,
    set: BitSet,
) -> Result<Option<UrResult>, ValidationResult> {
    let (row, col) = grid.compartments_containing((x, y));
    let base_set = set;
    if base_set.len() != 3 {
        return Ok(None);
    }

    for t in [row, col]
        .into_iter()
        .map(|c| c.to_unresolved())
        .filter(|c| c.len() == 2)
    {
        let (p, other_set) = t.iter().cloned().find(|(p, _c)| *p != (x, y)).unwrap();
        if base_set == other_set {
            /* implied that sets are continuous here */
            for (pos, cell) in [grid.get_row(y), grid.get_col(x)].into_iter().flatten() {
                if pos == (x, y) || pos == p {
                    continue;
                }
                if !base_set.intersection(cell.to_unresolved()).is_empty() {
                    return Ok(None);
                }
            }
            let numbers = base_set.into_iter().sorted().collect::<Vec<_>>();
            let middle = numbers[1];
            grid.set_impossible(p, middle)?;
            return Ok(Some(UrResult::SingleCellWouldBecomeFree(p, middle)));
        }
    }

    Ok(None)
}

type CPair = (Compartment, BitSet, Vec<Point>);
fn compartment_pairs(grid: &Grid) -> Vec<(CPair, CPair)> {
    let mut res = Vec::new();
    for compartment in grid.iter_by_compartments() {
        let vertical = compartment.vertical;
        let top_left = compartment.sample_pos();
        let base_set = compartment.combined_unresolved();

        if base_set.len() != 2 && base_set.len() != 3 {
            continue;
        }

        // todo: also implement compartments with multiple disjoint sets

        let unresolved_pos = compartment
            .cells
            .iter()
            .filter_map(|(pos, cell)| {
                if matches!(cell, Cell::Indeterminate(_)) {
                    Some(*pos)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if unresolved_pos.len() != 2 {
            continue;
        }

        'outer: for other in grid.iter_by_compartments() {
            if vertical != other.vertical {
                continue;
            }
            if (!vertical && other.sample_pos().1 <= top_left.1)
                || (vertical && other.sample_pos().0 <= top_left.0)
            {
                continue;
            }

            let other_set = other.combined_unresolved();

            let other_pos = other
                .cells
                .iter()
                .filter_map(|(pos, cell)| {
                    if matches!(cell, Cell::Indeterminate(_)) {
                        Some(*pos)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if other_pos.len() != unresolved_pos.len() {
                continue;
            }

            for &(x0, y0) in &unresolved_pos {
                if !other_pos
                    .iter()
                    .any(|&(x, y)| if vertical { y0 == y } else { x0 == x })
                {
                    continue 'outer;
                }
            }

            res.push((
                (compartment.clone(), base_set, unresolved_pos.clone()),
                (other, other_set, other_pos),
            ))
        }
    }
    res
}
fn get_set(grid: &Grid, a: Point, b: Point) -> BitSet {
    let a = grid.get_cell(a).to_unresolved();
    let b = grid.get_cell(b).to_unresolved();
    a.union(b)
}

/* If two compartments form a closed set except for one number, that one number must be contained
 * in the two containers, as otherwise there will be at least two solutions to the puzzle, eg.:
 *
 * [123]
 * [123] [12]
 * [123] [12]
 *
 * The [123] at (0, 0) cannot be 3, as the two other containers would form a closed set.
 */
fn two_compartments_would_have_closed_set(
    grid: &mut Grid,
) -> Result<Option<UrResult>, ValidationResult> {
    fn pair_set_candidate(grid: &Grid, a: Point, b: Point) -> bool {
        let a = grid.get_cell(a).to_unresolved();
        let b = grid.get_cell(b).to_unresolved();
        a.len() == 2 && a == b
    }
    fn get_coord(vertical: bool, pos: Point) -> usize {
        if vertical {
            pos.1
        } else {
            pos.0
        }
    }

    for ((compartment, base_set, unresolved_pos), (_other, other_set, other_pos)) in
        compartment_pairs(grid)
    {
        if base_set.union(other_set).len() != 3 || (base_set.len() == 2 && other_set.len() == 2) {
            continue;
        }
        let vertical = compartment.vertical;
        let cross_set_1 = get_set(grid, unresolved_pos[0], other_pos[0]);
        let cross_set_2 = get_set(grid, unresolved_pos[1], other_pos[1]);

        if unresolved_pos
            .iter()
            .map(|p| get_coord(vertical, *p))
            .collect::<Vec<_>>()
            != other_pos
                .iter()
                .map(|p| get_coord(vertical, *p))
                .collect::<Vec<_>>()
        {
            continue;
        }

        if base_set == other_set && base_set == cross_set_1 && base_set == cross_set_2 {
            continue;
        }

        if !pair_set_candidate(grid, unresolved_pos[0], unresolved_pos[1])
            && !pair_set_candidate(grid, other_pos[0], other_pos[1])
            && !pair_set_candidate(grid, unresolved_pos[0], other_pos[0])
            && !pair_set_candidate(grid, unresolved_pos[1], other_pos[1])
        {
            continue;
        }

        let impossible = base_set
            .symmetric_difference(other_set)
            .into_iter()
            .next()
            .or(cross_set_1
                .symmetric_difference(cross_set_2)
                .into_iter()
                .next())
            .unwrap();

        let mut changes = false;
        if pair_set_candidate(grid, unresolved_pos[0], unresolved_pos[1]) {
            changes |= grid.set_impossible_in(
                other_pos[0],
                vertical,
                impossible,
                &other_pos.iter().copied().collect(),
            )?;
        }
        if !changes && pair_set_candidate(grid, other_pos[0], other_pos[1]) {
            changes |= grid.set_impossible_in(
                unresolved_pos[0],
                vertical,
                impossible,
                &unresolved_pos.iter().copied().collect(),
            )?;
        }
        if !changes && pair_set_candidate(grid, unresolved_pos[0], other_pos[0]) {
            changes |= grid.set_impossible_in(
                unresolved_pos[1],
                !vertical,
                impossible,
                &[unresolved_pos[1], other_pos[1]].into_iter().collect(),
            )?;
        }
        if !changes && pair_set_candidate(grid, unresolved_pos[1], other_pos[1]) {
            changes |= grid.set_impossible_in(
                unresolved_pos[0],
                !vertical,
                impossible,
                &[unresolved_pos[0], other_pos[0]].into_iter().collect(),
            )?;
        }
        if changes {
            return Ok(Some(UrResult::ClosedSetCompartment(
                unresolved_pos
                    .into_iter()
                    .chain(other_pos.into_iter())
                    .collect(),
                impossible,
            )));
        }
    }

    Ok(None)
}

/* If two pairwise compartments would become ambiguous by removing a number from both,
 * that number has to exist in exactly one of them; and it must be also present in the
 * same row as the other container as otherwise the two compartments could be swapped, eg.:
 * [123] [123] # [3456] [3456]
 * [123] [123] # [3456] [3456]
 * [123456]
 * ...
 * The [123] containers must contain exactly one [12] and one [23] run. If the rows contain
 * only one 3 in total (in the containers), then the two compartments could be swapped.
 * This means we can add 3 to row requirements for both rows.
 */
fn two_compartment_setti(grid: &mut Grid) -> Result<Option<UrResult>, ValidationResult> {
    for ((compartment, base_set, unresolved_pos), (other, other_set, other_pos)) in
        compartment_pairs(grid)
    {
        if base_set.union(other_set).len() != 3 {
            continue;
        }
        let vertical = compartment.vertical;
        let cross_set_1 = get_set(grid, unresolved_pos[0], other_pos[0]);
        let cross_set_2 = get_set(grid, unresolved_pos[1], other_pos[1]);
        if base_set != other_set || base_set != cross_set_1 || base_set != cross_set_2 {
            continue;
        }

        let sample_pos = compartment.sample_pos();
        let other_sample_pos = other.sample_pos();

        let min = base_set.into_iter().min().unwrap();
        let max = base_set.into_iter().max().unwrap();

        let line_1 = if vertical {
            grid.get_col(sample_pos.0)
        } else {
            grid.get_row(sample_pos.1)
        };
        let line_2 = if vertical {
            grid.get_col(other_sample_pos.0)
        } else {
            grid.get_row(other_sample_pos.1)
        };
        let compartments = [line_1, line_2];

        let mut contains_min = false;
        let mut contains_max = false;
        for other in compartments
            .into_iter()
            .flat_map(|c| Grid::line_to_compartments(vertical, c).into_iter())
        {
            if other.contains_pos(sample_pos) || other.contains_pos(other_sample_pos) {
                continue;
            }

            contains_min |= other.combined_unresolved().contains(min);
            contains_max |= other.combined_unresolved().contains(max);
        }

        let to_add = if contains_min && !contains_max {
            min
        } else if !contains_min && contains_max {
            max
        } else {
            continue;
        };

        let mut changes = false;
        changes |= grid.requirements_mut(vertical, sample_pos).insert(max);
        changes |= grid
            .requirements_mut(vertical, other_sample_pos)
            .insert(max);

        if changes {
            return Ok(Some(UrResult::UrSetti(
                unresolved_pos
                    .into_iter()
                    .chain(other_pos.into_iter())
                    .collect(),
                vertical,
                to_add,
            )));
        }
    }

    Ok(None)
}

fn will_have_closed_sets(grid: &mut Grid) -> Result<bool, ValidationResult> {
    crate::strats::trivial(grid);
    while crate::strats::update_impossibles(grid)? {
        crate::strats::trivial(grid);
    }
    while crate::strats::definite_min_max(grid)? {
        crate::strats::trivial(grid);
    }
    for ((_compartment, base_set, unresolved_pos), (_other, other_set, other_pos)) in
        compartment_pairs(grid)
    {
        if unresolved_pos.len() == 2
            && other_pos.len() == 2
            && base_set == other_set
            && base_set.len() == other_set.len()
            && unresolved_pos.len() == base_set.len()
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/* Setting a cell to be some number causes the grid to immediately contain
 * closed sets, causing ambiguity:
 *
 * [23]   [1]   [23]
 * [1234] [234] [1234]
 *
 * Setting (1, 1) to be 4 causes the remaining cells to be [23] pairs.
 */
fn solution_causes_closed_sets(grid: &mut Grid) -> Result<Option<UrResult>, ValidationResult> {
    for (pos, set) in grid.iter_by_indeterminates() {
        for num in set {
            let mut subgrid = grid.clone();
            subgrid.set_cell(pos, Cell::Solution(num));
            if let Ok(true) = will_have_closed_sets(&mut subgrid) {
                grid.set_impossible(pos, num)?;
                return Ok(Some(UrResult::SolutionCausesClosedSets(pos, num)));
            }
        }
    }
    Ok(None)
}

pub fn unique_requirement(
    grid: &mut Grid,
    enable_guesses: bool,
) -> Result<Option<UrResult>, ValidationResult> {
    for ((x, y), cell) in grid.iter_by_cells() {
        if let Cell::Indeterminate(set) = cell {
            if let Some(res) = single_cell_cross_compartment_unique(grid, x, y, set)? {
                return Ok(Some(res));
            }
            if let Some(res) = single_cell_would_become_free(grid, x, y, set)? {
                return Ok(Some(res));
            }
            if let Some(res) = single_cell_intra_compartment_unique(grid, x, y, set)? {
                return Ok(Some(res));
            }
        }
    }

    if let Some(res) = two_compartments_would_have_closed_set(grid)? {
        return Ok(Some(res));
    }

    if let Some(res) = two_compartment_setti(grid)? {
        return Ok(Some(res));
    }

    if enable_guesses {
        if let Some(res) = solution_causes_closed_sets(grid)? {
            return Ok(Some(res));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Cell;
    use crate::solver::solve_basic;
    use crate::solver::SolveResults::OutOfBasicStrats;
    use crate::strats::update_required_and_forbidden;
    use crate::utils::*;

    #[test]
    fn test_cross_compartment() {
        let mut grid = g("
#######
##..#.#
#######
##..#.#
##..#.#
#######
#######
");
        grid.cells[1][2] = det([1, 2, 3]);
        grid.cells[1][3] = det([1, 2, 3]);

        grid.cells[1][5] = det([1, 5]);

        grid.cells[3][5] = det([1, 2, 3]);
        grid.cells[4][5] = det([1, 2, 3]);

        grid.cells[3][2] = det([1, 2, 3, 4]);
        grid.cells[3][3] = det([1, 2, 3, 4]);
        grid.cells[4][2] = det([1, 2, 3, 4]);
        grid.cells[4][3] = det([1, 2, 3, 4]);

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(
            unique_requirement(&mut grid, true),
            Ok(Some(UrResult::SingleUnique((5, 1), 5)))
        );

        assert_eq!(grid.cells[1][2], det([1, 2, 3]));
        assert_eq!(grid.cells[1][3], det([1, 2, 3]));
        assert_eq!(grid.cells[1][5], Cell::Solution(5));
        assert_eq!(grid.cells[3][5], det([1, 2, 3]));
        assert_eq!(grid.cells[4][5], det([1, 2, 3]));
    }

    #[test]
    fn test_intra_compartment() {
        let mut grid = g("
#########
#...#....
#...#....
#........
##..#####
#........
#........
#........
#########
");
        set_range(&mut grid, (1, 1), (3, 3), [1, 2, 3, 4]);
        set_range(&mut grid, (5, 1), (8, 3), [4, 5, 6, 7, 8]);
        set_range(&mut grid, (1, 5), (1, 7), [4, 5, 6, 7]);
        grid.cells[1][1] = det([2, 3]);
        grid.cells[1][3] = det([2, 3]);
        grid.cells[2][2] = det([2, 3, 4]);
        grid.cells[3][1] = det([2, 3, 4]);

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(
            unique_requirement(&mut grid, true),
            Ok(Some(UrResult::IntraCompartmentUnique((1, 2), 4)))
        );

        assert_eq!(grid.cells[2][1], det([1, 2, 3]));
    }

    #[test]
    fn test_two_compartment() {
        let mut grid = g("
#######
#..####
#..####
##....#
###...#
###...#
#######
");
        grid.cells[1][1] = det([1, 3, 4]);
        grid.cells[1][2] = det([2, 3, 4]);
        grid.cells[2][1] = det([3, 4]);
        grid.cells[2][2] = det([3, 4]);
        grid.cells[3][2] = det([2, 3, 4, 5]);

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(
            unique_requirement(&mut grid, true),
            Ok(Some(UrResult::ClosedSetCompartment(
                vec![(1, 1), (2, 1), (1, 2), (2, 2)],
                2
            )))
        );

        assert_eq!(grid.cells[3][2], det([3, 4, 5]));
    }

    #[test]
    fn test_two_compartment_intra() {
        let mut grid = g("
#########
#.1.#####
#...#####
#.......#
##......#
#.......#
#.......#
#.......#
#.......#
");
        grid.cells[1][1] = det([2, 3]);
        grid.cells[1][3] = det([2, 3]);
        grid.cells[3][3] = det([9]);
        set_range(&mut grid, (1, 2), (3, 2), [1, 2, 3, 4]);
        set_range(&mut grid, (1, 3), (3, 2), [2, 4, 6]);

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(
            unique_requirement(&mut grid, true),
            Ok(Some(UrResult::SolutionCausesClosedSets((2, 2), 4)))
        );

        assert_eq!(grid.cells[2][2], det([2, 3]));
    }

    #[test]
    fn compartment_would_have_free_cell() {
        let mut grid = g("
#######
#.#####
#.....#
##....#
##....#
##....#
#######
");
        set_range(&mut grid, (1, 1), (2, 2), [1, 2, 3]);
        grid.cells[1][2] = Cell::Black;

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(
            unique_requirement(&mut grid, true),
            Ok(Some(UrResult::SingleCellWouldBecomeFree((1, 2), 2)))
        );

        assert_eq!(grid.cells[2][1], det([1, 3]));
    }

    #[test]
    fn two_compartment_setti() {
        let mut grid = g("
#######
#..#..#
#..#..#
#.....#
#.....#
#.....#
#######
    ");
        set_range(&mut grid, (1, 1), (2, 2), [1, 2, 3]);
        set_range(&mut grid, (4, 1), (6, 2), [3, 4, 5, 6, 7]);

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert!(!grid.row_requirements[1].contains(3));
        assert!(!grid.row_requirements[2].contains(3));

        assert_eq!(
            unique_requirement(&mut grid, true),
            Ok(Some(UrResult::UrSetti(
                vec![(1, 1), (2, 1), (1, 2), (2, 2)],
                false,
                3
            )))
        );

        assert!(grid.row_requirements[1].contains(3));
        assert!(grid.row_requirements[2].contains(3));
    }
}
