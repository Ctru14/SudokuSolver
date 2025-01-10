use std::default;

#[derive(Default, Debug, Clone)]
pub struct SudokuSquare {
    pub value: Option<u32>,
    pub options: Vec<u32>,
    pub given: bool,
}

#[derive(Debug, Clone)]
pub struct Sudoku {
    pub grid: Vec<Vec<SudokuSquare>>,
}

impl Default for Sudoku {
    fn default() -> Self {
        Sudoku {
            grid: med_board_1(),
            // grid: vec![vec![SudokuSquare::default(); 9]; 9],
        }
    }
}

impl Sudoku {
    // Solve the puzzle! Public function called on solve button
    pub fn solve(&mut self) {
        // Solve the Sudoku puzzle
        
        let mut iteration_change: bool = true;

        while iteration_change {
            iteration_change = false;

            // Loop through each square in the grid
            for r in 0..9 {
                for c in 0..9 {
                    if self.grid[r][c].value.is_none() {
                        // Solve 1: Update square if row, column, or box has only one option
                        self.update_options(r, c);
                        if self.grid[r][c].options.len() == 1 {
                            self.grid[r][c].value = Some(self.grid[r][c].options[0]);
                            iteration_change = true;
                        }

                        // Solve 2: Update square if only square in row, column, or box with option
                        // Todo
                    }
                }
            }
        }
    }

    fn update_options(&mut self, r: usize, c: usize) {
        // List the possible options for a given square
        
        // Find options for the row, column, and 3x3 box
        let row_options: Vec<u32> = self.row_options(r);
        let col_options: Vec<u32> = self.col_options(c);
        let box_options: Vec<u32> = self.box_options(r, c);
        
        self.grid[r][c].options = Sudoku::combine_options(row_options, col_options, box_options);
    }

    // List all the available open numbers in the row
    fn row_options(&self, r: usize) -> Vec<u32> {
        let mut options: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 0..9 {
            // Loop through the row and remove any options values that are present in the row
            if self.grid[r][i].value.is_some() {
                let value: u32 = self.grid[r][i].value.unwrap();
                options.retain(|&x| x != value);
            }
        }
        options
    }

    // List all the available open numbers in the column
    fn col_options(&self, c: usize) -> Vec<u32> {
        let mut options: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 0..9 {
            // Loop through the row and remove any options values that are present in the row
            if self.grid[i][c].value.is_some() {
                let value: u32 = self.grid[i][c].value.unwrap();
                options.retain(|&x| x != value);
            }
        }
        options
    }

    // List all the available open numbers in the 3x3 box
    fn box_options(&self, r: usize, c: usize) -> Vec<u32> {
        let mut options: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let r_start = r - r % 3;
        let c_start = c - c % 3;
        for i in r_start..r_start + 3 {
            for j in c_start..c_start + 3 {
                if self.grid[i][j].value.is_some() {
                    let value: u32 = self.grid[i][j].value.unwrap();
                    options.retain(|&x| x != value);
                }
            }
        }
        options
    }

    // Combine the options from the row, column, and box, returning only the options common to all three
    fn combine_options(row_options: Vec<u32>, col_options: Vec<u32>, box_options: Vec<u32>) -> Vec<u32> {
        // Combine the options from the row, column, and box
        let mut options: Vec<u32> = vec![];
        for i in 1u32..=9 {
            if row_options.contains(&i) && col_options.contains(&i) && box_options.contains(&i) {
                options.push(i);
            }
        }
        options
    }

    // 
}


// ------------------------ Default boards ------------------------

// Shorthand for creating a new SudokuSquare struct
fn sqr(value: u32) -> SudokuSquare {
    if value >= 1 && value <= 9 {
        SudokuSquare { value: Some(value), options: Vec::default(), given: true }
    } else {
        SudokuSquare { value: None, options: Vec::default(), given: false }
    }
}


pub fn med_board_1() -> Vec<Vec<SudokuSquare>> {
    vec![
        vec![ sqr(0), sqr(1), sqr(0), /**/ sqr(0), sqr(0), sqr(8), /**/ sqr(0), sqr(9), sqr(0) ],
        vec![ sqr(0), sqr(0), sqr(0), /**/ sqr(0), sqr(0), sqr(1), /**/ sqr(0), sqr(8), sqr(7) ],
        vec![ sqr(0), sqr(7), sqr(8), /**/ sqr(0), sqr(0), sqr(0), /**/ sqr(1), sqr(0), sqr(0) ],
        /**************************************************************************************/
        vec![ sqr(8), sqr(0), sqr(7), /**/ sqr(0), sqr(1), sqr(0), /**/ sqr(3), sqr(0), sqr(2) ],
        vec![ sqr(0), sqr(0), sqr(0), /**/ sqr(8), sqr(2), sqr(4), /**/ sqr(0), sqr(0), sqr(0) ],
        vec![ sqr(4), sqr(0), sqr(9), /**/ sqr(0), sqr(6), sqr(0), /**/ sqr(5), sqr(0), sqr(8) ],
        /**************************************************************************************/
        vec![ sqr(0), sqr(0), sqr(4), /**/ sqr(0), sqr(0), sqr(0), /**/ sqr(6), sqr(5), sqr(0) ],
        vec![ sqr(2), sqr(8), sqr(0), /**/ sqr(3), sqr(0), sqr(0), /**/ sqr(0), sqr(0), sqr(0) ],
        vec![ sqr(0), sqr(5), sqr(0), /**/ sqr(1), sqr(0), sqr(0), /**/ sqr(0), sqr(2), sqr(0) ],
    ]
}