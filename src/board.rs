pub mod board {

    use std::cmp::max;
    use std::collections::HashSet;

    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Coord {
        pub col: usize,
        pub row: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Cell {
        Empty,
        Occupied
    }

    pub struct Board {
        cells: Vec<Vec<Cell>>,
        occupied: HashSet<Coord>,
    }

    impl Board {

        pub fn new(width: usize, height: usize) -> Board {

            // minimum board size is 1x1
            let cols = max(width, 1);
            let rows = max(height, 1);

            let mut tmp: Vec<Vec<Cell>> = Vec::new();

            for _ in 0..rows {
                tmp.push(vec![Cell::Empty; cols]);
            }

            Board {cells: tmp, occupied: HashSet::new()}

        }

        pub fn ensure_cell(&mut self, col: usize, row: usize) {

            // extend board by 1 if needed, no need to extend more
            // because we always scan vicinity of radius 1 of any cell

            if row as isize - self.cells.len() as isize > 0 {
                panic!("Row index is {} but the number of rows is {}",
                        row, self.cells.len());
            }

            if col as isize - self.cells[row].len() as isize > 0 {
                panic!("Col index is {} but the number of cols is {}",
                        col, self.cells[row].len());
            }

            if row >= self.cells.len() {
                self.cells.push(Vec::new());
            }

            if col >= self.cells[row].len() {
                self.cells[row].push(Cell::Empty);
            }

        }

        pub fn born_at(&mut self, col: usize, row: usize) {
            self.ensure_cell(col, row);
            self.cells[row][col] = Cell::Occupied;

            self.occupied.insert(Coord {col: col, row: row});
        }

        pub fn kill_at(&mut self, col: usize, row: usize) {
            self.ensure_cell(col, row);
            self.cells[row][col] = Cell::Empty;

            self.occupied.remove(&Coord {col: col, row: row});
        }

        pub fn get_cell(&mut self, col: usize, row: usize) -> Cell {
            self.ensure_cell(col, row);
            self.cells[row][col]
        }

        pub fn get_occupied<'a>(&'a self) -> Box<Iterator<Item=&'a Coord> + 'a> {
            Box::new(self.occupied.iter())
        }

    }

}


#[test]
fn test_board_ok() {

    use std::collections::HashSet;

    let mut my_board = board::Board::new(5, 5);

    // set some existing cells
    my_board.born_at(0, 0);
    my_board.born_at(4, 4);

    // extend board by one cell
    my_board.born_at(5, 2);

    // test allocated cells
    assert_eq!(my_board.get_cell(0, 0), board::Cell::Occupied);
    assert_eq!(my_board.get_cell(4, 4), board::Cell::Occupied);

    // test previously expanded cell
    assert_eq!(my_board.get_cell(5, 2), board::Cell::Occupied);

    // test existing cell
    assert_eq!(my_board.get_cell(2, 2), board::Cell::Empty);

    // check extended cell
    assert_eq!(my_board.get_cell(5, 3), board::Cell::Empty);

    my_board.kill_at(0, 0);
    assert_eq!(my_board.get_cell(0, 0), board::Cell::Empty);

    let mut expected: HashSet<board::Coord> = HashSet::new();

    expected.insert(board::Coord{col: 5, row: 2});
    expected.insert(board::Coord{col: 4, row: 4});

    let tmp = my_board.get_occupied().collect::<Vec<&board::Coord>>();

    assert_eq!(tmp.contains(&&board::Coord{col: 5, row: 2}), true);
    assert_eq!(tmp.contains(&&board::Coord{col: 4, row: 4}), true);
    assert_eq!(tmp.len(), 2);

}

#[test]
#[should_panic]
fn test_board_panic_extend() {

    let mut my_board = board::Board::new(5, 5);

    // can't extend board more than 1 cell
    my_board.get_cell(3, 6);

}
