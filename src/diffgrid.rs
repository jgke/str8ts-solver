use solver::bitset::BitSet;
use solver::grid::{Cell, Grid};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffGrid {
    pub cells: Vec<Vec<(Cell, Cell)>>,
    pub x: usize,
    pub y: usize,
    pub row_requirements: Vec<(BitSet, BitSet)>,
    pub col_requirements: Vec<(BitSet, BitSet)>,
    pub row_forbidden: Vec<(BitSet, BitSet)>,
    pub col_forbidden: Vec<(BitSet, BitSet)>,
}

impl DiffGrid {
    fn req_diff(left: Vec<BitSet>, right: Option<Vec<BitSet>>) -> Vec<(BitSet, BitSet)> {
        if let Some(right) = right {
            let r = &right;
            left.into_iter()
                .enumerate()
                .map(|(idx, s)| (s, r[idx]))
                .collect()
        } else {
            left.into_iter().map(|s| (s, s)).collect()
        }
    }

    pub fn new(left: Grid, right: Option<Grid>) -> DiffGrid {
        let row_requirements = Self::req_diff(
            left.row_requirements,
            right.as_ref().map(|r| r.row_requirements.clone()),
        );
        let col_requirements = Self::req_diff(
            left.col_requirements,
            right.as_ref().map(|r| r.col_requirements.clone()),
        );
        let row_forbidden = Self::req_diff(
            left.row_forbidden,
            right.as_ref().map(|r| r.row_forbidden.clone()),
        );
        let col_forbidden = Self::req_diff(
            left.col_forbidden,
            right.as_ref().map(|r| r.col_forbidden.clone()),
        );
        if let Some(right) = right {
            let r = &right;
            DiffGrid {
                cells: left
                    .cells
                    .into_iter()
                    .enumerate()
                    .map(|(y, row)| {
                        row.into_iter()
                            .enumerate()
                            .map(move |(x, c)| (c, r.cells[y][x].clone()))
                            .collect()
                    })
                    .collect(),
                x: left.x,
                y: left.y,
                row_requirements,
                col_requirements,
                row_forbidden,
                col_forbidden,
            }
        } else {
            DiffGrid {
                cells: left
                    .cells
                    .into_iter()
                    .map(|row| row.into_iter().map(move |c| (c.clone(), c)).collect())
                    .collect(),
                x: left.x,
                y: left.y,
                row_requirements,
                col_requirements,
                row_forbidden,
                col_forbidden,
            }
        }
    }
}
