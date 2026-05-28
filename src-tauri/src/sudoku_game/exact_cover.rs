mod dlx;

use self::dlx::DlxMatrix;
use super::sudoku_constants::{BOX_SIDE, GRID_SIDE, VALUE_MAX, VALUE_MIN};
const ASSIGNMENT_ROW_COUNT: usize = GRID_SIDE * GRID_SIDE * GRID_SIDE;
const CONSTRAINT_FAMILY_COUNT: usize = 4;
const CONSTRAINTS_PER_ASSIGNMENT: usize = CONSTRAINT_FAMILY_COUNT;
const CONSTRAINT_COLUMN_COUNT: usize = GRID_SIDE * GRID_SIDE * CONSTRAINT_FAMILY_COUNT;
const CONSTRAINT_COLUMNS_PER_FAMILY: usize = GRID_SIDE * GRID_SIDE;
#[cfg(test)]
const CELL_COUNT: usize = GRID_SIDE * GRID_SIDE;
const CELL_CONSTRAINT_OFFSET: usize = 0;
const ROW_VALUE_CONSTRAINT_OFFSET: usize =
    CELL_CONSTRAINT_OFFSET + CONSTRAINT_COLUMNS_PER_FAMILY;
const COLUMN_VALUE_CONSTRAINT_OFFSET: usize =
    ROW_VALUE_CONSTRAINT_OFFSET + CONSTRAINT_COLUMNS_PER_FAMILY;
const BOX_VALUE_CONSTRAINT_OFFSET: usize =
    COLUMN_VALUE_CONSTRAINT_OFFSET + CONSTRAINT_COLUMNS_PER_FAMILY;

// Public API
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ExactCoverError {
    InvalidRow(usize),
    InvalidColumn(usize),
    InvalidValue(u8),
    #[cfg(test)]
    InvalidAssignmentRowIndex(usize),
    #[cfg(test)]
    InvalidConstraintColumnIndex(usize),
    #[cfg(test)]
    InvalidConstraintSlot(usize),
    InvalidCap(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Assignment {
    row: usize,
    column: usize,
    value: u8,
}

impl Assignment {
    pub(super) fn new(row: usize, column: usize, value: u8) -> Result<Self, ExactCoverError> {
        validate_row(row)?;
        validate_column(column)?;
        validate_value(value)?;
        Ok(Self { row, column, value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ExactCoverAnalyzer {
    forced_rows: [bool; ASSIGNMENT_ROW_COUNT],
    covered_columns: [bool; CONSTRAINT_COLUMN_COUNT],
    fixed_values: [[Option<u8>; GRID_SIDE]; GRID_SIDE],
    assigned_cells: usize,
    contradiction: bool,
}

impl ExactCoverAnalyzer {
    pub(super) fn new(clues: &[Assignment]) -> Self {
        let mut analyzer = Self {
            forced_rows: [false; ASSIGNMENT_ROW_COUNT],
            covered_columns: [false; CONSTRAINT_COLUMN_COUNT],
            fixed_values: [[None; GRID_SIDE]; GRID_SIDE],
            assigned_cells: 0,
            contradiction: false,
        };

        for &clue in clues {
            analyzer.apply_clue(clue);
        }

        analyzer
    }

    pub(super) fn has_contradiction(&self) -> bool {
        self.contradiction
    }

    pub(super) fn count_solutions_with_cap(&self, cap: usize) -> Result<usize, ExactCoverError> {
        self.count_solutions_with_optional_assignment(cap, None)
    }

    pub(super) fn candidate_masks(&self) -> Result<[[u16; GRID_SIDE]; GRID_SIDE], ExactCoverError> {
        let mut masks = [[0u16; GRID_SIDE]; GRID_SIDE];
        for row in 0..GRID_SIDE {
            for column in 0..GRID_SIDE {
                masks[row][column] = self.candidate_mask_for_cell(row, column)?;
            }
        }
        Ok(masks)
    }

    // Private utility methods
    fn candidate_is_valid(
        &self,
        row: usize,
        column: usize,
        value: u8,
    ) -> Result<bool, ExactCoverError> {
        if self.contradiction {
            return Ok(false);
        }

        let assignment = Assignment::new(row, column, value)?;
        let count = self.count_solutions_with_optional_assignment(1, Some(assignment))?;
        Ok(count > 0)
    }

    fn candidate_mask_for_cell(
        &self,
        row: usize,
        column: usize,
    ) -> Result<u16, ExactCoverError> {
        validate_row(row)?;
        validate_column(column)?;

        if self.contradiction {
            return Ok(0);
        }

        let mut mask: u16 = 0;
        for value in VALUE_MIN..=VALUE_MAX {
            if self.candidate_is_valid(row, column, value)? {
                mask |= value_to_bit(value);
            }
        }

        Ok(mask)
    }

    fn apply_clue(&mut self, clue: Assignment) {
        if self.contradiction {
            return;
        }

        let row_index = assignment_to_row_index(clue);
        if self.forced_rows[row_index] {
            return;
        }

        match self.fixed_values[clue.row][clue.column] {
            Some(existing) if existing != clue.value => {
                self.contradiction = true;
                return;
            }
            None => {
                self.fixed_values[clue.row][clue.column] = Some(clue.value);
                self.assigned_cells += 1;
            }
            _ => {}
        }

        let columns = assignment_to_constraint_columns(clue);
        if columns.iter().any(|&column| self.covered_columns[column]) {
            self.contradiction = true;
            return;
        }

        self.forced_rows[row_index] = true;
        for column in columns {
            self.covered_columns[column] = true;
        }
    }

    fn count_solutions_with_optional_assignment(
        &self,
        cap: usize,
        extra_assignment: Option<Assignment>,
    ) -> Result<usize, ExactCoverError> {
        if cap == 0 {
            return Err(ExactCoverError::InvalidCap(cap));
        }
        if self.contradiction {
            return Ok(0);
        }

        let mut forced_rows = self.forced_rows;
        let mut covered_columns = self.covered_columns;

        if let Some(extra) = extra_assignment {
            let row_index = assignment_to_row_index(extra);
            if !forced_rows[row_index] {
                if let Some(existing) = self.fixed_values[extra.row][extra.column] {
                    if existing != extra.value {
                        return Ok(0);
                    }
                }

                let columns = assignment_to_constraint_columns(extra);
                if columns.iter().any(|&column| covered_columns[column]) {
                    return Ok(0);
                }

                forced_rows[row_index] = true;
                for column in columns {
                    covered_columns[column] = true;
                }
            }
        }

        let mut column_map = [usize::MAX; CONSTRAINT_COLUMN_COUNT];
        let mut remaining_columns = 0;
        for column in 0..CONSTRAINT_COLUMN_COUNT {
            if !covered_columns[column] {
                column_map[column] = remaining_columns;
                remaining_columns += 1;
            }
        }

        if remaining_columns == 0 {
            return Ok(1);
        }

        let mut matrix =
            DlxMatrix::new(remaining_columns).expect("remaining_columns is non-zero and valid");
        let problem = build_full_exact_cover_problem();
        for row_index in 0..ASSIGNMENT_ROW_COUNT {
            if forced_rows[row_index] {
                continue;
            }

            let columns = problem.rows[row_index];
            if columns.iter().any(|&column| covered_columns[column]) {
                continue;
            }

            let mut mapped_columns = [0usize; CONSTRAINTS_PER_ASSIGNMENT];
            for (slot, &column) in columns.iter().enumerate() {
                mapped_columns[slot] = column_map[column];
            }
            matrix
                .add_row(&mapped_columns)
                .expect("column mapping preserves uniqueness and bounds");
        }

        Ok(matrix
            .count_solutions_with_cap(cap)
            .expect("cap already validated before calling DLX solver"))
    }

    // Test-only helpers
    #[cfg(test)]
    fn assigned_cells(&self) -> usize {
        self.assigned_cells
    }

    #[cfg(test)]
    fn progress(&self) -> BoardProgress {
        if !self.contradiction && self.assigned_cells == CELL_COUNT {
            return BoardProgress::SolvedNow;
        }
        BoardProgress::UnsolvedNow
    }

    #[cfg(test)]
    fn solution_count_status(
        &self,
        cap: usize,
    ) -> Result<SolutionCountStatus, ExactCoverError> {
        let count = self.count_solutions_with_cap(cap)?;
        classify_solution_count(count, cap)
    }
}

// Private utility functions
fn assignment_to_row_index(assignment: Assignment) -> usize {
    ((assignment.row * GRID_SIDE + assignment.column) * GRID_SIDE)
        + usize::from(assignment.value - VALUE_MIN)
}

#[cfg(test)]
fn row_index_to_assignment(row_index: usize) -> Result<Assignment, ExactCoverError> {
    if row_index >= ASSIGNMENT_ROW_COUNT {
        return Err(ExactCoverError::InvalidAssignmentRowIndex(row_index));
    }

    let row_column_index = row_index / GRID_SIDE;
    let row = row_column_index / GRID_SIDE;
    let column = row_column_index % GRID_SIDE;
    let value = (row_index % GRID_SIDE) as u8 + VALUE_MIN;
    Assignment::new(row, column, value)
}

fn assignment_to_constraint_columns(
    assignment: Assignment,
) -> [usize; CONSTRAINTS_PER_ASSIGNMENT] {
    let cell = cell_constraint_column_formula(assignment.row, assignment.column);
    let row_value = row_value_constraint_column_formula(assignment.row, assignment.value);
    let column_value = column_value_constraint_column_formula(assignment.column, assignment.value);
    let box_value = box_value_constraint_column_formula(
        box_index(assignment.row, assignment.column),
        assignment.value,
    );
    [cell, row_value, column_value, box_value]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
enum ConstraintFamily {
    Cell,
    RowValue,
    ColumnValue,
    BoxValue,
}

#[cfg(test)]
impl ConstraintFamily {
    fn offset(self) -> usize {
        match self {
            ConstraintFamily::Cell => CELL_CONSTRAINT_OFFSET,
            ConstraintFamily::RowValue => ROW_VALUE_CONSTRAINT_OFFSET,
            ConstraintFamily::ColumnValue => COLUMN_VALUE_CONSTRAINT_OFFSET,
            ConstraintFamily::BoxValue => BOX_VALUE_CONSTRAINT_OFFSET,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
enum ConstraintColumn {
    Cell { row: usize, column: usize },
    RowValue { row: usize, value: u8 },
    ColumnValue { column: usize, value: u8 },
    BoxValue { box_index: usize, value: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
enum SolutionCountStatus {
    Contradiction,
    Unique,
    MultipleBelowCap(usize),
    AtLeastCap(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
enum BoardProgress {
    SolvedNow,
    UnsolvedNow,
}

#[cfg(test)]
fn constraint_column_to_descriptor(
    column_index: usize,
) -> Result<ConstraintColumn, ExactCoverError> {
    if column_index >= CONSTRAINT_COLUMN_COUNT {
        return Err(ExactCoverError::InvalidConstraintColumnIndex(column_index));
    }

    let family = column_index / CONSTRAINT_COLUMNS_PER_FAMILY;
    let local = column_index % CONSTRAINT_COLUMNS_PER_FAMILY;
    let major = local / GRID_SIDE;
    let minor = local % GRID_SIDE;
    let value = minor as u8 + VALUE_MIN;

    match family {
        0 => Ok(ConstraintColumn::Cell {
            row: major,
            column: minor,
        }),
        1 => Ok(ConstraintColumn::RowValue { row: major, value }),
        2 => Ok(ConstraintColumn::ColumnValue {
            column: major,
            value,
        }),
        3 => Ok(ConstraintColumn::BoxValue {
            box_index: major,
            value,
        }),
        _ => Err(ExactCoverError::InvalidConstraintColumnIndex(column_index)),
    }
}

#[cfg(test)]
fn constraint_column_from_family_slot(
    family: ConstraintFamily,
    slot: usize,
) -> Result<usize, ExactCoverError> {
    if slot >= CONSTRAINT_COLUMNS_PER_FAMILY {
        return Err(ExactCoverError::InvalidConstraintSlot(slot));
    }
    Ok(family.offset() + slot)
}

fn build_full_exact_cover_problem() -> ExactCoverProblem {
    let mut problem = ExactCoverProblem::new();
    for row in 0..GRID_SIDE {
        for column in 0..GRID_SIDE {
            for value in VALUE_MIN..=VALUE_MAX {
                let assignment =
                    Assignment::new(row, column, value).expect("loop values are in range");
                problem
                    .rows
                    .push(assignment_to_constraint_columns(assignment));
            }
        }
    }

    debug_assert_eq!(problem.rows.len(), ASSIGNMENT_ROW_COUNT);
    problem
}

#[cfg(test)]
fn classify_solution_count(
    count: usize,
    cap: usize,
) -> Result<SolutionCountStatus, ExactCoverError> {
    if cap < 2 {
        return Err(ExactCoverError::InvalidCap(cap));
    }

    if count == 0 {
        return Ok(SolutionCountStatus::Contradiction);
    }
    if count == 1 {
        return Ok(SolutionCountStatus::Unique);
    }
    if count >= cap {
        return Ok(SolutionCountStatus::AtLeastCap(cap));
    }
    Ok(SolutionCountStatus::MultipleBelowCap(count))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExactCoverProblem {
    column_count: usize,
    rows: Vec<[usize; CONSTRAINTS_PER_ASSIGNMENT]>,
}

impl ExactCoverProblem {
    pub fn new() -> Self {
        Self {
            column_count: CONSTRAINT_COLUMN_COUNT,
            rows: Vec::with_capacity(ASSIGNMENT_ROW_COUNT),
        }
    }
}

impl Default for ExactCoverProblem {
    fn default() -> Self {
        Self::new()
    }
}

fn value_to_bit(value: u8) -> u16 {
    1u16 << u32::from(value - VALUE_MIN)
}

#[cfg(test)]
fn bit_to_value(bit: u16) -> Option<u8> {
    if bit.count_ones() != 1 {
        return None;
    }
    let index = bit.trailing_zeros() as u8;
    let value = index + VALUE_MIN;
    if value > VALUE_MAX {
        return None;
    }
    Some(value)
}

fn cell_constraint_column_formula(row: usize, column: usize) -> usize {
    CELL_CONSTRAINT_OFFSET + row * GRID_SIDE + column
}

fn row_value_constraint_column_formula(row: usize, value: u8) -> usize {
    ROW_VALUE_CONSTRAINT_OFFSET + row * GRID_SIDE + usize::from(value - VALUE_MIN)
}

fn column_value_constraint_column_formula(column: usize, value: u8) -> usize {
    COLUMN_VALUE_CONSTRAINT_OFFSET + column * GRID_SIDE + usize::from(value - VALUE_MIN)
}

fn box_value_constraint_column_formula(box_index: usize, value: u8) -> usize {
    BOX_VALUE_CONSTRAINT_OFFSET + box_index * GRID_SIDE + usize::from(value - VALUE_MIN)
}

fn validate_row(row: usize) -> Result<(), ExactCoverError> {
    if row >= GRID_SIDE {
        return Err(ExactCoverError::InvalidRow(row));
    }
    Ok(())
}

fn validate_column(column: usize) -> Result<(), ExactCoverError> {
    if column >= GRID_SIDE {
        return Err(ExactCoverError::InvalidColumn(column));
    }
    Ok(())
}

fn validate_value(value: u8) -> Result<(), ExactCoverError> {
    if !(VALUE_MIN..=VALUE_MAX).contains(&value) {
        return Err(ExactCoverError::InvalidValue(value));
    }
    Ok(())
}

fn box_index(row: usize, column: usize) -> usize {
    (row / BOX_SIDE) * BOX_SIDE + (column / BOX_SIDE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solved_grid_assignments() -> Vec<Assignment> {
        let rows = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];

        let mut assignments = Vec::with_capacity(CELL_COUNT);
        for (row, values) in rows.iter().enumerate() {
            for (column, &value) in values.iter().enumerate() {
                assignments.push(Assignment::new(row, column, value).unwrap());
            }
        }
        assignments
    }

    #[test]
    fn assignment_rejects_out_of_range_inputs() {
        assert_eq!(
            Assignment::new(GRID_SIDE, 0, VALUE_MIN),
            Err(ExactCoverError::InvalidRow(GRID_SIDE))
        );
        assert_eq!(
            Assignment::new(0, GRID_SIDE, VALUE_MIN),
            Err(ExactCoverError::InvalidColumn(GRID_SIDE))
        );
        assert_eq!(
            Assignment::new(0, 0, 0),
            Err(ExactCoverError::InvalidValue(0))
        );
        assert_eq!(
            Assignment::new(0, 0, VALUE_MAX + 1),
            Err(ExactCoverError::InvalidValue(VALUE_MAX + 1))
        );
    }

    #[test]
    fn classify_solution_count_matches_status_model() {
        assert_eq!(
            classify_solution_count(0, 100),
            Ok(SolutionCountStatus::Contradiction)
        );
        assert_eq!(
            classify_solution_count(1, 100),
            Ok(SolutionCountStatus::Unique)
        );
        assert_eq!(
            classify_solution_count(2, 100),
            Ok(SolutionCountStatus::MultipleBelowCap(2))
        );
        assert_eq!(
            classify_solution_count(100, 100),
            Ok(SolutionCountStatus::AtLeastCap(100))
        );
    }

    #[test]
    fn assignment_row_index_round_trips_exhaustively() {
        let mut seen = vec![false; ASSIGNMENT_ROW_COUNT];
        for row in 0..GRID_SIDE {
            for column in 0..GRID_SIDE {
                for value in VALUE_MIN..=VALUE_MAX {
                    let assignment = Assignment::new(row, column, value).unwrap();
                    let row_index = assignment_to_row_index(assignment);
                    assert!(row_index < ASSIGNMENT_ROW_COUNT);
                    assert!(!seen[row_index]);
                    seen[row_index] = true;
                    assert_eq!(row_index_to_assignment(row_index).unwrap(), assignment);
                }
            }
        }

        assert!(seen.iter().all(|was_seen| *was_seen));
        assert_eq!(
            row_index_to_assignment(ASSIGNMENT_ROW_COUNT),
            Err(ExactCoverError::InvalidAssignmentRowIndex(
                ASSIGNMENT_ROW_COUNT
            ))
        );
    }

    #[test]
    fn constraint_column_mappings_hold_exhaustively() {
        let mut hit_counts = vec![0usize; CONSTRAINT_COLUMN_COUNT];

        for row in 0..GRID_SIDE {
            for column in 0..GRID_SIDE {
                for value in VALUE_MIN..=VALUE_MAX {
                    let assignment = Assignment::new(row, column, value).unwrap();
                    let columns = assignment_to_constraint_columns(assignment);

                    assert_eq!(columns.len(), CONSTRAINTS_PER_ASSIGNMENT);
                    assert!(columns[0] != columns[1]);
                    assert!(columns[0] != columns[2]);
                    assert!(columns[0] != columns[3]);
                    assert!(columns[1] != columns[2]);
                    assert!(columns[1] != columns[3]);
                    assert!(columns[2] != columns[3]);

                    for &column_index in &columns {
                        assert!(column_index < CONSTRAINT_COLUMN_COUNT);
                        hit_counts[column_index] += 1;
                    }

                    assert_eq!(
                        constraint_column_to_descriptor(columns[0]).unwrap(),
                        ConstraintColumn::Cell { row, column }
                    );
                    assert_eq!(
                        constraint_column_to_descriptor(columns[1]).unwrap(),
                        ConstraintColumn::RowValue { row, value }
                    );
                    assert_eq!(
                        constraint_column_to_descriptor(columns[2]).unwrap(),
                        ConstraintColumn::ColumnValue { column, value }
                    );
                    assert_eq!(
                        constraint_column_to_descriptor(columns[3]).unwrap(),
                        ConstraintColumn::BoxValue {
                            box_index: box_index(row, column),
                            value
                        }
                    );
                }
            }
        }

        for count in hit_counts {
            assert_eq!(count, GRID_SIDE);
        }

        assert_eq!(
            constraint_column_to_descriptor(CONSTRAINT_COLUMN_COUNT),
            Err(ExactCoverError::InvalidConstraintColumnIndex(
                CONSTRAINT_COLUMN_COUNT
            ))
        );
    }

    #[test]
    fn builds_full_problem_with_expected_dimensions() {
        let problem = build_full_exact_cover_problem();
        assert_eq!(problem.column_count, CONSTRAINT_COLUMN_COUNT);
        assert_eq!(problem.rows.len(), ASSIGNMENT_ROW_COUNT);
        for row in problem.rows {
            assert_eq!(row.len(), CONSTRAINTS_PER_ASSIGNMENT);
        }
    }

    #[test]
    fn family_slot_round_trips_and_checks_bounds() {
        let slot = 42;
        let column = constraint_column_from_family_slot(ConstraintFamily::RowValue, slot).unwrap();
        assert_eq!(column, ROW_VALUE_CONSTRAINT_OFFSET + slot);
        let column_value_column =
            constraint_column_from_family_slot(ConstraintFamily::ColumnValue, slot).unwrap();
        assert_eq!(column_value_column, COLUMN_VALUE_CONSTRAINT_OFFSET + slot);
        let box_value_column =
            constraint_column_from_family_slot(ConstraintFamily::BoxValue, slot).unwrap();
        assert_eq!(box_value_column, BOX_VALUE_CONSTRAINT_OFFSET + slot);
        assert_eq!(
            constraint_column_from_family_slot(
                ConstraintFamily::Cell,
                CONSTRAINT_COLUMNS_PER_FAMILY
            ),
            Err(ExactCoverError::InvalidConstraintSlot(
                CONSTRAINT_COLUMNS_PER_FAMILY
            ))
        );
    }

    #[test]
    fn analyzer_detects_contradicting_clues() {
        let clues = vec![
            Assignment::new(0, 0, 1).unwrap(),
            Assignment::new(0, 1, 1).unwrap(),
        ];
        let analyzer = ExactCoverAnalyzer::new(&clues);
        assert!(analyzer.has_contradiction());
        assert_eq!(analyzer.progress(), BoardProgress::UnsolvedNow);
        assert_eq!(
            analyzer.solution_count_status(100),
            Ok(SolutionCountStatus::Contradiction)
        );
    }

    #[test]
    fn analyzer_reports_unique_for_complete_valid_grid() {
        let clues = solved_grid_assignments();
        let analyzer = ExactCoverAnalyzer::new(&clues);
        assert!(!analyzer.has_contradiction());
        assert_eq!(analyzer.assigned_cells(), CELL_COUNT);
        assert_eq!(analyzer.progress(), BoardProgress::SolvedNow);
        assert_eq!(analyzer.count_solutions_with_cap(100), Ok(1));
        assert_eq!(
            analyzer.solution_count_status(100),
            Ok(SolutionCountStatus::Unique)
        );

        let mask = analyzer.candidate_mask_for_cell(0, 0).unwrap();
        assert_eq!(mask, value_to_bit(5));
        assert_eq!(bit_to_value(mask), Some(5));
    }

    #[test]
    fn analyzer_cap_reports_at_least_for_empty_grid() {
        let analyzer = ExactCoverAnalyzer::new(&[]);
        assert!(!analyzer.has_contradiction());
        assert_eq!(analyzer.progress(), BoardProgress::UnsolvedNow);
        assert_eq!(
            analyzer.solution_count_status(2),
            Ok(SolutionCountStatus::AtLeastCap(2))
        );
    }

    #[test]
    fn candidate_validation_rejects_conflicting_entry() {
        let clues = vec![Assignment::new(0, 0, 5).unwrap()];
        let analyzer = ExactCoverAnalyzer::new(&clues);
        assert_eq!(analyzer.candidate_is_valid(0, 0, 6), Ok(false));
        assert_eq!(analyzer.candidate_is_valid(0, 1, 5), Ok(false));
        assert_eq!(analyzer.candidate_is_valid(0, 1, 3), Ok(true));
    }
}
