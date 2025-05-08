use crate::bitset::BitSet;
use crate::grid::{Grid, Point};
use crate::solver::ValidationResult;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

fn indeterminates_matching(grid: &Grid, (x, y): Point) -> Vec<(Point, BitSet)> {
    grid.iter_by_indeterminates()
        .into_iter()
        .filter(|&((xx, yy), _)| (x == xx) || (y == yy))
        .collect()
}

type Pairs = HashMap<(Point, u8), BTreeSet<Point>>;
fn gather_pairs(grid: &mut Grid) -> Pairs {
    let mut res = HashMap::new();

    for (pos, set) in grid.iter_by_indeterminates() {
        for num in set {
            let mut other_row = vec![];
            let mut other_col = vec![];
            for (other_pos, other_set) in indeterminates_matching(grid, pos) {
                if other_pos == pos {
                    continue;
                }
                if !grid.requirements(other_pos.0 == pos.0, pos).contains(num) {
                    continue;
                }
                if other_set.contains(num) {
                    if other_pos.0 == pos.0 {
                        other_col.push(other_pos);
                    } else {
                        other_row.push(other_pos);
                    }
                }
            }

            for other in [other_row, other_col]
                .into_iter()
                .filter(|vec| vec.len() == 1)
                .flatten()
            {
                res.entry((pos, num))
                    .or_insert_with(BTreeSet::new)
                    .insert(other);
                res.entry((other, num))
                    .or_insert_with(BTreeSet::new)
                    .insert(pos);
            }
        }
    }

    res
}

fn next_from_hashset<V>(set: &mut BTreeSet<V>) -> Option<V>
where
    V: Eq + Ord + Hash + Copy,
{
    set.iter().next().copied().inspect(|v| {
        set.remove(v);
    })
}

#[allow(clippy::type_complexity)]
fn split(colors: &Colors) -> (Vec<(Point, u8)>, Vec<(Point, u8)>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for (&pos, nums) in colors {
        for (&num, &val) in nums {
            if val {
                right.push((pos, num));
            } else {
                left.push((pos, num));
            }
        }
    }
    left.sort();
    right.sort();
    (left, right)
}

type Colors = BTreeMap<Point, BTreeMap<u8, bool>>;
fn get_colors(grid: &mut Grid, pairs: &Pairs, orig_pos: Point, orig_num: u8) -> Colors {
    let mut colors: Colors = BTreeMap::new();
    let mut queue: BTreeSet<Point> = BTreeSet::new();
    colors.insert(orig_pos, [(orig_num, false)].into_iter().collect());
    for &pos in pairs.get(&(orig_pos, orig_num)).into_iter().flatten() {
        colors.entry(pos).or_default().insert(orig_num, true);
        queue.insert(pos);
    }

    while let Some(pos) = next_from_hashset(&mut queue) {
        let prev_set = grid.get_cell(pos).to_possibles();

        if colors.entry(pos).or_default().len() == 1 && prev_set.len() == 2 {
            let existing = colors[&pos].clone().into_iter().next().unwrap();
            let new = prev_set.into_iter().find(|&num| num != existing.0).unwrap();
            colors.get_mut(&pos).unwrap().insert(new, !existing.1);
        }

        for (num, color) in colors[&pos].clone() {
            for &pos in pairs.get(&(pos, num)).into_iter().flatten() {
                if colors.entry(pos).or_default().insert(num, !color).is_none() {
                    queue.insert(pos);
                }
            }
        }
    }
    colors
}

type SeenColor = HashMap<u8, Vec<Point>>;
fn get_row_and_col_colors(
    colors: &Colors,
    center: Point,
    include_center: bool,
) -> (SeenColor, SeenColor) {
    let mut seen_false: SeenColor = HashMap::new();
    let mut seen_true: SeenColor = HashMap::new();

    for (&pos, cell_colors) in colors {
        if center == pos && !include_center {
            continue;
        }
        if pos.0 != center.0 && pos.1 != center.1 {
            continue;
        }

        for (&num, &color) in cell_colors {
            if color {
                seen_true.entry(num).or_default().push(pos);
            } else {
                seen_false.entry(num).or_default().push(pos);
            }
        }
    }

    (seen_false, seen_true)
}

fn split_seen(seen: &SeenColor, center: Point) -> (SeenColor, SeenColor) {
    let seen_col = seen
        .iter()
        .map(|(&k, v)| {
            (
                k,
                v.iter()
                    .copied()
                    .filter(|&(x, _)| x == center.0)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    let seen_row = seen
        .iter()
        .map(|(&k, v)| {
            (
                k,
                v.iter()
                    .copied()
                    .filter(|&(_, y)| y == center.1)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    (seen_row, seen_col)
}

fn block_color(grid: &mut Grid, colors: &Colors, val: bool) -> Result<bool, ValidationResult> {
    let mut changes = false;
    for (&pos, cell_colors) in colors {
        for num in cell_colors
            .iter()
            .filter(|(_, b)| **b == val)
            .map(|(n, _)| *n)
        {
            changes |= grid.set_impossible(pos, num)?;
        }
    }
    Ok(changes)
}

#[allow(clippy::type_complexity)]
pub fn medusa(
    grid: &mut Grid,
) -> Result<Option<(Vec<(Point, u8)>, Vec<(Point, u8)>)>, ValidationResult> {
    let mut previously_colored = HashSet::new();
    let pairs = gather_pairs(grid);
    for (orig_pos, orig_set) in grid.iter_by_indeterminates() {
        for orig_num in orig_set {
            if previously_colored.contains(&(orig_pos, orig_num)) {
                continue;
            }
            let colors = get_colors(grid, &pairs, orig_pos, orig_num);
            for (&pos, map) in &colors {
                for &num in map.keys() {
                    previously_colored.insert((pos, num));
                }
            }
            if colors.len() == 1 {
                continue;
            }

            let mut res = None;
            let mut changes = false;

            /* Case 1: multiple cells with same color in set -> that color is illegal */
            for cell_colors in colors.values() {
                let trues = cell_colors.values().filter(|val| **val).count();
                let falses = cell_colors.values().filter(|val| !*val).count();

                if trues > 1 {
                    res = Some(true);
                    break;
                }
                if falses > 1 {
                    res = Some(false);
                    break;
                }
            }
            if let Some(val) = res {
                if block_color(grid, &colors, val)? {
                    return Ok(Some(split(&colors)));
                }
            }

            /* Case 2: multiple cells with same number and color in row or col -> that color is illegal */
            'outer: for (&pos, cell_colors) in &colors {
                let (seen_false, seen_true) = get_row_and_col_colors(&colors, pos, true);
                let (seen_false_row, seen_false_col) = split_seen(&seen_false, pos);
                let (seen_true_row, seen_true_col) = split_seen(&seen_true, pos);

                for &num in cell_colors.keys() {
                    if seen_false_row.get(&num).map(|vec| vec.len() > 1) == Some(true)
                        || seen_false_col.get(&num).map(|vec| vec.len() > 1) == Some(true)
                    {
                        res = Some(false);
                        break 'outer;
                    }
                    if seen_true_row.get(&num).map(|vec| vec.len() > 1) == Some(true)
                        || seen_true_col.get(&num).map(|vec| vec.len() > 1) == Some(true)
                    {
                        res = Some(true);
                        break 'outer;
                    }
                }
            }
            if let Some(val) = res {
                if block_color(grid, &colors, val)? {
                    return Ok(Some(split(&colors)));
                }
            }

            /* Case 3: Two colors in a cell -> other values are not possible */
            for (&pos, cell_colors) in &colors {
                let possibles = grid.get_cell(pos).to_possibles();
                let colored = cell_colors.keys().copied().collect::<BitSet>();
                if colored.len() > 1 {
                    for num in possibles.difference(colored) {
                        changes |= grid.set_impossible(pos, num)?;
                    }
                }
            }
            if changes {
                return Ok(Some(split(&colors)));
            }

            /* Case 4: Uncolored cell number sees same number in both colors -> it can be removed */
            for (pos, set) in grid.iter_by_indeterminates() {
                let (seen_false, seen_true) = get_row_and_col_colors(&colors, pos, false);
                for num in set {
                    if colors.get(&pos).and_then(|m| m.get(&num)).is_some() {
                        continue;
                    }

                    if seen_false.contains_key(&num) && seen_true.contains_key(&num) {
                        changes |= grid.set_impossible(pos, num)?;
                    }
                }
            }
            if changes {
                return Ok(Some(split(&colors)));
            }

            /* Case 5: Uncolored cell number sees same number colored in other cell and shares cell
             * with other color -> it can be removed */
            for (pos, set) in grid.iter_by_indeterminates() {
                let (seen_false, seen_true) = get_row_and_col_colors(&colors, pos, false);
                for num in set {
                    if colors.get(&pos).and_then(|m| m.get(&num)).is_some() {
                        continue;
                    }

                    /* there is zero or one colors in cell here -- two is handled in Case 3 */
                    if let Some(shared) = colors
                        .get(&pos)
                        .and_then(|map| map.values().copied().next())
                    {
                        if (shared && seen_false.contains_key(&num))
                            || (!shared && seen_true.contains_key(&num))
                        {
                            changes |= grid.set_impossible(pos, num)?;
                        }
                    }
                }
            }
            if changes {
                return Ok(Some(split(&colors)));
            }

            /* Case 6: Cell with no colors sees a same-colored candidate for all of its cells
             * -> all colored can be removed as they would empty the cell */
            for (pos, set) in grid.iter_by_indeterminates() {
                if colors.get(&pos).map(|m| !m.is_empty()).unwrap_or(false) {
                    continue;
                }
                let (seen_false, seen_true) = get_row_and_col_colors(&colors, pos, false);

                let seen_f = seen_false.keys().copied().collect::<BitSet>();
                let seen_t = seen_true.keys().copied().collect::<BitSet>();

                if seen_t == set {
                    changes |= block_color(grid, &colors, true)?;
                } else if seen_f == set {
                    changes |= block_color(grid, &colors, false)?;
                }
            }
            if changes {
                return Ok(Some(split(&colors)));
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
    use crate::strats::update_required_and_forbidden;
    use crate::utils::*;
    use rustc_hash::FxHashSet;

    pub fn set<const N: usize>(vals: [Point; N]) -> FxHashSet<Point> {
        vals.into_iter().collect()
    }

    #[test]
    fn test_medusa_rule_1() {
        let mut grid = g("
a...b....
.........
..#....##
....d.f..
c.....e..
.........
..#......
..#......
..#......
");
        let a = (0, 0);
        let b = (4, 0);
        let c = (0, 4);
        let d = (4, 3);
        let e = (6, 4);
        let f = (6, 3);

        grid.cells[a.1][a.0] = det([1, 4, 7, 8]);
        grid.cells[b.1][b.0] = det([4, 7]);
        grid.cells[c.1][c.0] = det([4, 5]);
        grid.cells[d.1][d.0] = det([5, 7]);
        grid.cells[e.1][e.0] = det([6, 5]);
        grid.cells[f.1][f.0] = det([5, 7, 8]);

        grid.set_impossible_in(a, false, 4, &set([a, b])).unwrap();
        grid.set_impossible_in(c, false, 5, &set([c, e])).unwrap();
        grid.set_impossible_in(d, false, 7, &set([d, f])).unwrap();

        grid.set_impossible_in(a, true, 4, &set([a, c])).unwrap();
        grid.set_impossible_in(b, true, 7, &set([b, d])).unwrap();
        grid.set_impossible_in(e, true, 5, &set([e, f])).unwrap();

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        update_required_and_forbidden(&mut grid).unwrap();

        assert_eq!(grid.cells[f.1][f.0], det([5, 7, 8]));

        assert_eq!(
            Ok(Some((
                vec![(a, 4), (c, 5), (b, 7), (d, 5), (f, 5), (f, 7), (e, 6)],
                vec![(c, 4), (b, 4), (d, 7), (e, 5)]
            ))),
            medusa(&mut grid)
        );

        assert_eq!(grid.cells[a.1][a.0], det([1, 7, 8]));
        assert_eq!(grid.cells[b.1][b.0], det([4]));
        assert_eq!(grid.cells[c.1][c.0], det([4]));
        assert_eq!(grid.cells[d.1][d.0], det([7]));
        assert_eq!(grid.cells[e.1][e.0], det([5]));
        assert_eq!(grid.cells[f.1][f.0], det([8]));
    }

    #[test]
    fn test_medusa_rule_2_row() {
        let mut grid = g("
a...b....
.........
..#....##
....d.f..
c.....e..
.........
..#......
..#......
..#......
");
        let a = (0, 0);
        let b = (4, 0);
        let c = (0, 4);
        let d = (4, 3);
        let e = (6, 4);
        let f = (6, 3);

        grid.cells[a.1][a.0] = det([1, 4, 7, 8]);
        grid.cells[b.1][b.0] = det([4, 7]);
        grid.cells[c.1][c.0] = det([4, 5]);
        grid.cells[d.1][d.0] = det([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        grid.cells[e.1][e.0] = det([5, 7]);
        grid.cells[f.1][f.0] = det([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        grid.set_impossible_in(a, false, 4, &set([a, b])).unwrap();
        grid.set_impossible_in(c, false, 5, &set([c, e])).unwrap();

        grid.set_impossible_in(a, true, 4, &set([a, c])).unwrap();
        grid.set_impossible_in(b, true, 7, &set([b, d])).unwrap();
        grid.set_impossible_in(f, true, 7, &set([e, f])).unwrap();

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        update_required_and_forbidden(&mut grid).unwrap();

        assert_eq!(grid.cells[f.1][f.0], det([1, 2, 3, 4, 5, 6, 7, 8, 9]));

        assert_eq!(
            Ok(Some((
                vec![((0, 0), 4), ((0, 4), 5), ((4, 0), 7), ((6, 4), 7)],
                vec![
                    ((0, 4), 4),
                    ((4, 0), 4),
                    ((4, 3), 7),
                    ((6, 3), 7),
                    ((6, 4), 5)
                ]
            ))),
            medusa(&mut grid)
        );

        assert_eq!(grid.cells[a.1][a.0], det([1, 4, 7, 8]));
        assert_eq!(grid.cells[b.1][b.0], det([7]));
        assert_eq!(grid.cells[c.1][c.0], det([5]));
        assert_eq!(grid.cells[d.1][d.0], det([1, 2, 3, 4, 5, 6, 8, 9]));
        assert_eq!(grid.cells[e.1][e.0], det([7]));
        assert_eq!(grid.cells[f.1][f.0], det([1, 2, 3, 4, 5, 6, 8, 9]));
    }

    #[test]
    fn test_medusa_rule_2_col() {
        let mut grid = g("
a...b....
.........
..#....##
....d.f..
c.....e..
.........
..#......
..#......
..#......
");
        let a = (0, 0);
        let b = (4, 0);
        let c = (0, 4);
        let d = (4, 3);
        let e = (6, 4);
        let f = (6, 3);

        grid.cells[a.1][a.0] = det([1, 4, 7, 8]);
        grid.cells[b.1][b.0] = det([4, 7]);
        grid.cells[c.1][c.0] = det([4, 5]);
        grid.cells[d.1][d.0] = det([5, 7]);
        grid.cells[e.1][e.0] = det([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        grid.cells[f.1][f.0] = det([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        grid.set_impossible_in(a, false, 4, &set([a, b])).unwrap();
        grid.set_impossible_in(c, false, 5, &set([c, e])).unwrap();
        grid.set_impossible_in(d, false, 5, &set([d, f])).unwrap();

        grid.set_impossible_in(a, true, 4, &set([a, c])).unwrap();
        grid.set_impossible_in(b, true, 7, &set([b, d])).unwrap();

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        update_required_and_forbidden(&mut grid).unwrap();

        assert_eq!(grid.cells[3][6], det([1, 2, 3, 4, 5, 6, 7, 8, 9]));

        assert_eq!(
            Ok(Some((
                vec![((0, 0), 4), ((0, 4), 5), ((4, 0), 7), ((4, 3), 5)],
                vec![
                    ((0, 4), 4),
                    ((4, 0), 4),
                    ((4, 3), 7),
                    ((6, 3), 5),
                    ((6, 4), 5)
                ]
            ))),
            medusa(&mut grid)
        );

        assert_eq!(grid.cells[a.1][a.0], det([1, 4, 7, 8]));
        assert_eq!(grid.cells[b.1][b.0], det([7]));
        assert_eq!(grid.cells[c.1][c.0], det([5]));
        assert_eq!(grid.cells[d.1][d.0], det([5]));
        assert_eq!(grid.cells[e.1][e.0], det([1, 2, 3, 4, 6, 7, 8, 9]));
        assert_eq!(grid.cells[f.1][f.0], det([1, 2, 3, 4, 6, 7, 8, 9]));
    }

    #[test]
    fn test_medusa_rule_3() {
        let mut grid = g("
.....
.a.b.
.....
.....
.c.d.
");
        let a = (1, 1);
        let b = (3, 1);
        let c = (1, 4);
        let d = (3, 4);

        grid.cells[a.1][a.0] = det([1, 2, 3]);
        grid.cells[b.1][b.0] = det([2, 3, 4]);
        grid.cells[c.1][c.0] = det([2, 3]);
        grid.cells[d.1][d.0] = det([2, 3, 4]);

        grid.set_impossible_in(a, false, 2, &set([a, b])).unwrap();
        grid.set_impossible_in(c, false, 3, &set([c, d])).unwrap();

        grid.set_impossible_in(a, true, 2, &set([a, c])).unwrap();
        grid.set_impossible_in(b, true, 3, &set([b, d])).unwrap();

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        update_required_and_forbidden(&mut grid).unwrap();

        assert_eq!(grid.cells[b.1][b.0], det([2, 3, 4]));

        assert_eq!(
            Ok(Some((
                vec![((1, 1), 2), ((1, 4), 3), ((3, 1), 3)],
                vec![((1, 4), 2), ((3, 1), 2), ((3, 4), 3)]
            ))),
            medusa(&mut grid)
        );

        assert_eq!(grid.cells[a.1][a.0], det([1, 2, 3]));
        assert_eq!(grid.cells[b.1][b.0], det([2, 3]));
        assert_eq!(grid.cells[c.1][c.0], det([2, 3]));
        assert_eq!(grid.cells[d.1][d.0], det([2, 3, 4]));
    }

    #[test]
    fn test_medusa_rule_4() {
        let mut grid = g("
.....
.a.b.
.....
.....
.c.d.
");
        let a = (1, 1);
        let b = (3, 1);
        let c = (1, 4);
        let d = (3, 4);

        grid.cells[a.1][a.0] = det([2, 3]);
        grid.cells[b.1][b.0] = det([2, 3, 4]);
        grid.cells[c.1][c.0] = det([2, 4]);
        grid.cells[d.1][d.0] = det([2, 3, 4]);

        grid.set_impossible_in(a, false, 3, &set([a, b])).unwrap();
        grid.set_impossible_in(c, false, 2, &set([c, d])).unwrap();

        grid.set_impossible_in(a, true, 2, &set([a, c])).unwrap();
        grid.set_impossible_in(b, true, 2, &set([b, d])).unwrap();

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        update_required_and_forbidden(&mut grid).unwrap();

        assert_eq!(grid.cells[d.1][d.0], det([2, 3, 4]));
        assert_eq!(grid.cells[1][0], det([1, 2, 4, 5]));
        assert_eq!(grid.cells[1][2], det([1, 2, 4, 5]));
        assert_eq!(grid.cells[1][4], det([1, 2, 4, 5]));

        assert_eq!(
            Ok(Some((
                vec![((1, 1), 2), ((1, 4), 4), ((3, 4), 2)],
                vec![((1, 4), 2), ((3, 1), 2)]
            ))),
            medusa(&mut grid)
        );

        assert_eq!(grid.cells[1][0], det([1, 4, 5]));
        assert_eq!(grid.cells[1][2], det([1, 4, 5]));
        assert_eq!(grid.cells[1][4], det([1, 4, 5]));
        assert_eq!(grid.cells[a.1][a.0], det([2, 3]));
        assert_eq!(grid.cells[b.1][b.0], det([2, 3, 4]));
        assert_eq!(grid.cells[c.1][c.0], det([2, 4]));
        assert_eq!(grid.cells[d.1][d.0], det([2, 3, 4]));
    }

    #[test]
    fn test_medusa_rule_5() {
        let mut grid = g("
a...b....
.........
..#....##
....d.f..
c.....e..
.........
..#......
..#......
..#......
");
        let a = (0, 0);
        let b = (4, 0);
        let c = (0, 4);
        let d = (4, 3);
        let e = (6, 4);
        let f = (6, 3);

        grid.cells[a.1][a.0] = det([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        grid.cells[b.1][b.0] = det([3, 7]);
        grid.cells[c.1][c.0] = det([1, 3]);
        grid.cells[d.1][d.0] = det([7, 8]);
        grid.cells[e.1][e.0] = det([1, 8]);
        grid.cells[f.1][f.0] = det([1, 6, 7]);

        grid.set_impossible_in(a, false, 3, &set([a, b])).unwrap();
        grid.set_impossible_in(c, false, 1, &set([c, e])).unwrap();
        grid.set_impossible_in(d, false, 7, &set([d, f])).unwrap();

        grid.set_impossible_in(a, true, 3, &set([a, c])).unwrap();
        grid.set_impossible_in(b, true, 7, &set([b, d])).unwrap();

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        update_required_and_forbidden(&mut grid).unwrap();

        assert_eq!(
            Ok(Some((
                vec![
                    ((0, 0), 3),
                    ((0, 4), 1),
                    ((4, 0), 7),
                    ((4, 3), 8),
                    ((6, 3), 7),
                    ((6, 4), 8)
                ],
                vec![((0, 4), 3), ((4, 0), 3), ((4, 3), 7), ((6, 4), 1)]
            ))),
            medusa(&mut grid)
        );

        assert_eq!(grid.cells[a.1][a.0], det([1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert_eq!(grid.cells[b.1][b.0], det([3, 7]));
        assert_eq!(grid.cells[c.1][c.0], det([1, 3]));
        assert_eq!(grid.cells[d.1][d.0], det([7, 8]));
        assert_eq!(grid.cells[e.1][e.0], det([1, 8]));
        assert_eq!(grid.cells[f.1][f.0], det([6, 7]));
    }

    #[test]
    fn test_medusa_rule_6() {
        let mut grid = g("
a...b....
.........
..#....##
....d.f..
c.....e..
.........
..#......
..#......
..#......
");
        let a = (0, 0);
        let b = (4, 0);
        let c = (0, 4);
        let d = (4, 3);
        let e = (6, 4);
        let f = (6, 3);

        grid.cells[a.1][a.0] = det([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        grid.cells[b.1][b.0] = det([3, 7]);
        grid.cells[c.1][c.0] = det([1, 3]);
        grid.cells[d.1][d.0] = det([7, 8]);
        grid.cells[e.1][e.0] = det([1, 8]);
        grid.cells[f.1][f.0] = det([1, 7]);

        grid.set_impossible_in(a, false, 3, &set([a, b])).unwrap();
        grid.set_impossible_in(c, false, 1, &set([c, e])).unwrap();

        grid.set_impossible_in(a, true, 3, &set([a, c])).unwrap();
        grid.set_impossible_in(b, true, 7, &set([b, d])).unwrap();

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        update_required_and_forbidden(&mut grid).unwrap();

        assert_eq!(
            Ok(Some((
                vec![
                    ((0, 0), 3),
                    ((0, 4), 1),
                    ((4, 0), 7),
                    ((4, 3), 8),
                    ((6, 4), 8)
                ],
                vec![((0, 4), 3), ((4, 0), 3), ((4, 3), 7), ((6, 4), 1)]
            ))),
            medusa(&mut grid)
        );

        assert_eq!(grid.cells[a.1][a.0], det([1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert_eq!(grid.cells[b.1][b.0], det([7]));
        assert_eq!(grid.cells[c.1][c.0], det([1]));
        assert_eq!(grid.cells[d.1][d.0], det([8]));
        assert_eq!(grid.cells[e.1][e.0], det([8]));
        assert_eq!(grid.cells[f.1][f.0], det([1, 7]));
    }
}
