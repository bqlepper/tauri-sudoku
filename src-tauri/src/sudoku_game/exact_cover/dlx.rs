const ROOT: usize = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DlxError {
    InvalidColumnCount(usize),
    EmptyRow,
    ColumnOutOfRange { column: usize, column_count: usize },
    DuplicateColumn(usize),
    InvalidCap(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DlxMatrix {
    column_count: usize,
    rows: Vec<Vec<usize>>,
}

impl DlxMatrix {
    pub(super) fn new(column_count: usize) -> Result<Self, DlxError> {
        if column_count == 0 {
            return Err(DlxError::InvalidColumnCount(column_count));
        }

        Ok(Self {
            column_count,
            rows: Vec::new(),
        })
    }

    pub(super) fn add_row(&mut self, columns: &[usize]) -> Result<usize, DlxError> {
        if columns.is_empty() {
            return Err(DlxError::EmptyRow);
        }

        let mut seen = vec![false; self.column_count];
        for &column in columns {
            if column >= self.column_count {
                return Err(DlxError::ColumnOutOfRange {
                    column,
                    column_count: self.column_count,
                });
            }
            if seen[column] {
                return Err(DlxError::DuplicateColumn(column));
            }
            seen[column] = true;
        }

        self.rows.push(columns.to_vec());
        Ok(self.rows.len() - 1)
    }

    pub(super) fn count_solutions_with_cap(&self, cap: usize) -> Result<usize, DlxError> {
        if cap == 0 {
            return Err(DlxError::InvalidCap(cap));
        }

        let mut search = DlxSearch::new(self.column_count, &self.rows);
        Ok(search.count_solutions_with_cap(cap))
    }
}

#[derive(Debug, Clone)]
struct DlxNode {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    column: usize,
    row_id: usize,
}

impl DlxNode {
    fn new_header(index: usize) -> Self {
        Self {
            left: index,
            right: index,
            up: index,
            down: index,
            column: index,
            row_id: usize::MAX,
        }
    }

    fn new_data(column: usize, row_id: usize) -> Self {
        Self {
            left: 0,
            right: 0,
            up: 0,
            down: 0,
            column,
            row_id,
        }
    }
}

struct DlxSearch {
    nodes: Vec<DlxNode>,
    column_sizes: Vec<usize>,
    solution_rows: Vec<usize>,
    found_count: usize,
    cap: usize,
}

impl DlxSearch {
    fn new(column_count: usize, rows: &[Vec<usize>]) -> Self {
        let mut search = Self {
            nodes: Vec::new(),
            column_sizes: vec![0; column_count + 1],
            solution_rows: Vec::new(),
            found_count: 0,
            cap: 0,
        };

        search.initialize_headers(column_count);
        for (row_id, columns) in rows.iter().enumerate() {
            search.insert_row(row_id, columns);
        }

        search
    }

    fn count_solutions_with_cap(&mut self, cap: usize) -> usize {
        self.cap = cap;
        self.search();
        self.found_count
    }

    fn initialize_headers(&mut self, column_count: usize) {
        self.nodes.push(DlxNode::new_header(ROOT));

        for _ in 0..column_count {
            let header = self.nodes.len();
            self.nodes.push(DlxNode::new_header(header));
            self.link_header_before_root(header);
        }
    }

    fn link_header_before_root(&mut self, header: usize) {
        let left_of_root = self.nodes[ROOT].left;

        self.nodes[header].left = left_of_root;
        self.nodes[header].right = ROOT;
        self.nodes[left_of_root].right = header;
        self.nodes[ROOT].left = header;
    }

    fn insert_row(&mut self, row_id: usize, columns: &[usize]) {
        let mut first: Option<usize> = None;
        let mut previous: usize = 0;

        for &column in columns {
            let header = column + 1;
            let node = self.nodes.len();
            self.nodes.push(DlxNode::new_data(header, row_id));

            self.link_node_to_column_bottom(node, header);
            self.column_sizes[header] += 1;

            if let Some(first_node) = first {
                self.nodes[node].left = previous;
                self.nodes[node].right = first_node;
                self.nodes[previous].right = node;
                self.nodes[first_node].left = node;
            } else {
                first = Some(node);
                self.nodes[node].left = node;
                self.nodes[node].right = node;
            }

            previous = node;
        }
    }

    fn link_node_to_column_bottom(&mut self, node: usize, header: usize) {
        let bottom = self.nodes[header].up;
        self.nodes[node].up = bottom;
        self.nodes[node].down = header;
        self.nodes[node].column = header;

        self.nodes[bottom].down = node;
        self.nodes[header].up = node;
    }

    fn search(&mut self) {
        if self.found_count >= self.cap {
            return;
        }

        if self.nodes[ROOT].right == ROOT {
            self.found_count += 1;
            return;
        }

        let column = self.choose_column_with_min_size();
        if self.column_sizes[column] == 0 {
            return;
        }

        self.cover(column);

        let mut row = self.nodes[column].down;
        while row != column {
            self.solution_rows.push(self.nodes[row].row_id);

            let mut right = self.nodes[row].right;
            while right != row {
                let column_to_cover = self.nodes[right].column;
                self.cover(column_to_cover);
                right = self.nodes[right].right;
            }

            self.search();

            let mut left = self.nodes[row].left;
            while left != row {
                let column_to_uncover = self.nodes[left].column;
                self.uncover(column_to_uncover);
                left = self.nodes[left].left;
            }

            self.solution_rows.pop();

            if self.found_count >= self.cap {
                break;
            }

            row = self.nodes[row].down;
        }

        self.uncover(column);
    }

    fn choose_column_with_min_size(&self) -> usize {
        let mut best = self.nodes[ROOT].right;
        let mut best_size = self.column_sizes[best];

        let mut cursor = self.nodes[best].right;
        while cursor != ROOT {
            let size = self.column_sizes[cursor];
            if size < best_size {
                best = cursor;
                best_size = size;
                if best_size == 0 {
                    break;
                }
            }
            cursor = self.nodes[cursor].right;
        }

        best
    }

    fn cover(&mut self, column: usize) {
        let left = self.nodes[column].left;
        let right = self.nodes[column].right;
        self.nodes[left].right = right;
        self.nodes[right].left = left;

        let mut row = self.nodes[column].down;
        while row != column {
            let mut node = self.nodes[row].right;
            while node != row {
                let up = self.nodes[node].up;
                let down = self.nodes[node].down;
                self.nodes[up].down = down;
                self.nodes[down].up = up;
                let node_column = self.nodes[node].column;
                self.column_sizes[node_column] -= 1;
                node = self.nodes[node].right;
            }
            row = self.nodes[row].down;
        }
    }

    fn uncover(&mut self, column: usize) {
        let mut row = self.nodes[column].up;
        while row != column {
            let mut node = self.nodes[row].left;
            while node != row {
                let node_column = self.nodes[node].column;
                self.column_sizes[node_column] += 1;
                let up = self.nodes[node].up;
                let down = self.nodes[node].down;
                self.nodes[up].down = node;
                self.nodes[down].up = node;
                node = self.nodes[node].left;
            }
            row = self.nodes[row].up;
        }

        let left = self.nodes[column].left;
        let right = self.nodes[column].right;
        self.nodes[left].right = column;
        self.nodes[right].left = column;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_row_validates_columns() {
        let mut matrix = DlxMatrix::new(10).unwrap();
        assert_eq!(matrix.add_row(&[]), Err(DlxError::EmptyRow));
        assert_eq!(
            matrix.add_row(&[10]),
            Err(DlxError::ColumnOutOfRange {
                column: 10,
                column_count: 10
            })
        );
        assert_eq!(matrix.add_row(&[1, 1]), Err(DlxError::DuplicateColumn(1)));
        assert_eq!(matrix.add_row(&[1, 3, 5]), Ok(0));
    }

    #[test]
    fn count_solutions_detects_contradiction() {
        let mut matrix = DlxMatrix::new(3).unwrap();
        matrix.add_row(&[0]).unwrap();
        matrix.add_row(&[1]).unwrap();
        assert_eq!(matrix.count_solutions_with_cap(10), Ok(0));
    }

    #[test]
    fn count_solutions_finds_unique_solution() {
        let mut matrix = DlxMatrix::new(3).unwrap();
        matrix.add_row(&[0]).unwrap();
        matrix.add_row(&[1]).unwrap();
        matrix.add_row(&[2]).unwrap();
        assert_eq!(matrix.count_solutions_with_cap(10), Ok(1));
    }

    #[test]
    fn count_solutions_respects_cap_and_stops_early() {
        let mut matrix = DlxMatrix::new(3).unwrap();
        matrix.add_row(&[0]).unwrap();
        matrix.add_row(&[1]).unwrap();
        matrix.add_row(&[2]).unwrap();
        matrix.add_row(&[0, 1, 2]).unwrap();
        assert_eq!(matrix.count_solutions_with_cap(1), Ok(1));
        assert_eq!(matrix.count_solutions_with_cap(2), Ok(2));
        assert_eq!(matrix.count_solutions_with_cap(10), Ok(2));
    }

    #[test]
    fn count_solutions_rejects_zero_cap() {
        let matrix = DlxMatrix::new(1).unwrap();
        assert_eq!(
            matrix.count_solutions_with_cap(0),
            Err(DlxError::InvalidCap(0))
        );
    }
}
