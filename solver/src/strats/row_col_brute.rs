use crate::bitset::BitSet;
use crate::grid::{Cell, Compartment, Grid};
use crate::validator::compartment_valid;

pub fn solution_valid(compartments: &[Compartment], requirements: BitSet) -> bool {
    let mut seen_numbers = BitSet::new();
    for compartment in compartments {
        for (_, cell) in &compartment.cells {
            match cell {
                Cell::Black => {}
                Cell::Indeterminate(_) => {}
                Cell::Solution(n) | Cell::Blocker(n) | Cell::Requirement(n) => {
                    if seen_numbers.contains(*n) {
                        return false;
                    }
                    seen_numbers.insert(*n);
                }
            }
        }
    }

    if !requirements.is_subset(seen_numbers) {
        return false;
    }

    compartments
        .iter()
        .all(|compartment| compartment_valid(compartment).is_ok())
}

pub fn compartment_solutions(
    compartments: &[Compartment],
    requirements: BitSet,
) -> Vec<Vec<Compartment>> {
    let mut available_solutions = Vec::new();
    let mut all_solved = true;
    'outer: for (index, compartment) in compartments.iter().enumerate() {
        for (cell_index, cell) in compartment.cells.iter().enumerate() {
            if let (loc, Cell::Indeterminate(set)) = cell {
                if set.is_empty() {
                    return vec![];
                }
                all_solved = false;
                for n in *set {
                    let mut with_solution = compartments.to_vec();
                    with_solution[index].cells[cell_index] = (*loc, Cell::Solution(n));
                    for compartment in &mut with_solution {
                        for i in 0..compartment.cells.len() {
                            if let Cell::Indeterminate(set) = &mut compartment.cells[i].1 {
                                set.remove(n);
                            }
                        }
                    }
                    if compartment_valid(&with_solution[index]).is_ok() {
                        available_solutions
                            .append(&mut compartment_solutions(&with_solution, requirements))
                    }
                }
                break 'outer;
            }
        }
    }
    if all_solved && solution_valid(compartments, requirements) {
        available_solutions.push(compartments.to_vec());
    }
    available_solutions
}

fn compartment_contains_number(comp: &Compartment, num: u8) -> bool {
    comp.cells.iter().any(|(_, cell)| match cell {
        Cell::Solution(n) | Cell::Requirement(n) => *n == num,
        Cell::Indeterminate(_) => unreachable!(),
        Cell::Black => false,
        Cell::Blocker(_) => false,
    })
}

fn compartments_contains_number(compartments: &[Compartment], num: u8) -> bool {
    compartments
        .iter()
        .any(|comp| compartment_contains_number(comp, num))
}

pub fn row_col_brute(grid: &mut Grid) -> bool {
    let mut changes = false;

    for (index, compartments) in grid.iter_by_row_compartments().into_iter().enumerate() {
        if compartments.len() <= 1 {
            continue;
        }
        let solutions = compartment_solutions(&compartments, grid.row_requirements[index]);
        if solutions.is_empty() {
            unreachable!();
        }
        for i in 1..=9 {
            if solutions
                .iter()
                .all(|comps| compartments_contains_number(comps, i))
            {
                changes |= grid.row_requirements[index].insert(i);
            }
            if solutions
                .iter()
                .all(|comps| !compartments_contains_number(comps, i))
            {
                changes |= grid.row_forbidden[index].insert(i);
            }
        }
    }

    for (index, compartments) in grid.iter_by_col_compartments().into_iter().enumerate() {
        if compartments.len() <= 1 {
            continue;
        }
        let solutions = compartment_solutions(&compartments, grid.col_requirements[index]);
        if solutions.is_empty() {
            unreachable!();
        }
        for i in 1..=9 {
            if solutions
                .iter()
                .all(|comps| compartments_contains_number(comps, i))
            {
                changes |= grid.col_requirements[index].insert(i);
            }
            if solutions
                .iter()
                .all(|comps| !compartments_contains_number(comps, i))
            {
                changes |= grid.col_forbidden[index].insert(i);
            }
        }
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strats::update_required_and_forbidden;
    use crate::utils::*;

    #[test]
    fn test_row_col_brute() {
        let mut grid = g("
.#..#
#####
.####
.####
#####
");

        grid.cells[0][0] = Cell::Indeterminate(set([1, 3]));
        grid.cells[0][2] = Cell::Indeterminate(set([1, 2, 3]));
        grid.cells[0][3] = Cell::Indeterminate(set([1, 2, 3]));
        grid.cells[2][0] = Cell::Indeterminate(set([1, 2, 3]));
        grid.cells[3][0] = Cell::Indeterminate(set([1, 2, 3]));

        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));

        assert_eq!(grid.row_requirements[0].len(), 1);
        assert!(grid.row_requirements[0].contains(2));
        assert_eq!(grid.col_requirements[0].len(), 1);
        assert!(grid.col_requirements[0].contains(2));
        assert!(!grid.col_forbidden[0].contains(4));

        assert_eq!(row_col_brute(&mut grid), true);

        assert!(grid.row_requirements[0].contains(1));
        assert!(grid.row_requirements[0].contains(2));
        assert!(grid.row_requirements[0].contains(3));

        assert!(grid.col_requirements[0].contains(1));
        assert!(grid.col_requirements[0].contains(2));
        assert!(grid.col_requirements[0].contains(3));
        assert!(grid.col_forbidden[0].contains(4));
    }
}
