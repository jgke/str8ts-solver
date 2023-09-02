use crate::grid::Grid;
use crate::strats::{possible_numbers, required_numbers};

pub fn setti(grid: &mut Grid) -> bool {
    let mut changes = false;

    for n in 1..=grid.x as u8 {
        let mut row_min = 0;
        let mut row_max = 0;
        for (row_n, row) in grid.iter_by_rows().into_iter().enumerate() {
            if grid.row_requirements[row_n].contains(n) {
                row_min += 1;
            }
            if possible_numbers(&row).contains(n) {
                row_max += 1;
            }
        }

        let mut col_min = 0;
        let mut col_max = 0;
        for (col_n, col) in grid.iter_by_cols().into_iter().enumerate() {
            if grid.col_requirements[col_n].contains(n) {
                col_min += 1;
            }
            if possible_numbers(&col).contains(n) {
                col_max += 1;
            }
        }

        let res: Vec<_> = ((row_min.max(col_min))..=(row_max.min(col_max))).collect();

        if res.len() == 1 {
            let setti_count = res[0];

            if row_max == setti_count {
                for (y, row) in grid.iter_by_rows().into_iter().enumerate() {
                    if !possible_numbers(&row).contains(n) {
                        changes |= grid.row_forbidden[y].insert(n);
                    } else {
                        changes |= grid.row_requirements[y].insert(n);
                    }
                }
            } else if row_min == setti_count {
                for (y, row) in grid.iter_by_rows().into_iter().enumerate() {
                    if !required_numbers(grid, &row).contains(n) {
                        changes |= grid.row_forbidden[y].insert(n);
                    }
                }
            }

            if col_max == setti_count {
                for (x, col) in grid.iter_by_cols().into_iter().enumerate() {
                    if !possible_numbers(&col).contains(n) {
                        changes |= grid.col_forbidden[x].insert(n);
                    } else {
                        changes |= grid.col_requirements[x].insert(n);
                    }
                }
            } else if col_min == setti_count {
                for (x, col) in grid.iter_by_cols().into_iter().enumerate() {
                    if !required_numbers(grid, &col).contains(n) {
                        changes |= grid.col_forbidden[x].insert(n);
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
    use crate::solver::run_basic;
    use crate::solver::SolveResults::OutOfBasicStrats;
    use crate::strats::update_required_and_forbidden;
    use crate::utils::*;

    #[test]
    fn test_setti() {
        let mut grid = g("
.1...
.....
.....
.....
.#...
");

        while dbg!(run_basic(&mut grid)) != OutOfBasicStrats {
            println!("{}\n", grid);
            println!("{:?}\n", grid);
        }

        assert!(update_required_and_forbidden(&mut grid));

        assert!(setti(&mut grid));

        assert_eq!(grid.cells[4][0], det([1, 2, 4, 5]));
        assert_eq!(grid.cells[4][2], det([1, 2, 3, 4, 5]));
        assert_eq!(grid.cells[4][3], det([1, 2, 3, 4, 5]));
        assert_eq!(grid.cells[4][4], det([1, 2, 3, 4, 5]));

        while run_basic(&mut grid) != OutOfBasicStrats {}

        assert_eq!(grid.cells[4][0], det([1, 4]));
        assert_eq!(grid.cells[4][2], det([1, 2, 3, 4]));
        assert_eq!(grid.cells[4][3], det([1, 2, 3, 4]));
        assert_eq!(grid.cells[4][4], det([1, 2, 3, 4]));
    }
}
