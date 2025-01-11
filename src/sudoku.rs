use std::{default, iter};

#[derive(Default, Debug, Clone)]
pub struct SudokuSquare {
    pub value: Option<u32>,
    pub options: Vec<u32>,
    pub given: bool,
}

#[derive(Debug, Clone)]
pub struct Sudoku {
    pub grid: Vec<Vec<SudokuSquare>>,
    pub solved: bool,
    pub status: String,
}

impl Default for Sudoku {
    fn default() -> Self {
        Sudoku {
            grid: med_board_1(),
            // grid: vec![vec![SudokuSquare::default(); 9]; 9],
            solved: false,
            status: String::default(),
        }
    }
}

impl Sudoku {
    // Solve the puzzle! Public function called on solve button
    pub fn solve(&mut self, next: bool) {
        // Solve the Sudoku puzzle
        
        let mut iteration_change: bool = true;

        'iteration_loop: while iteration_change {
            iteration_change = false;

            // Loop Solve 1: Update square if row, column, or box has only one option
            for r in 0..9 {
                for c in 0..9 {
                    if self.grid[r][c].value.is_none() {
                        self.update_options(r, c);
                        if self.grid[r][c].options.len() == 1 {
                            self.set_square(r, c, self.grid[r][c].options[0], false);
                            println!("One square option: {} in space R={}, C={}", self.grid[r][c].value.unwrap(), r + 1, c + 1);
                            iteration_change = true;
                            if next { return; }
                            else { continue 'iteration_loop; }
                        }
                    }
                }
            }

            // Loop Solve 2: Update square if only square in row, column, or box with option
            // This is in a separate loop to ensure that all the options are updated before checking
            for r in 0..9 {
                for c in 0..9 {
                    if self.grid[r][c].value.is_none() {
                        iteration_change = self.update_only_options(r, c);
                        if iteration_change { 
                            if next { return; }
                            else { continue 'iteration_loop; }
                        } 
                    }
                }
            }
        }

        if next {
            println!("Solve Next unable to find next value");
        } 
        else {
            self.check_solved();
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

    // Determine if the square is the only one in its row with a certain option
    fn row_only_option(&self, r: usize, c: usize) -> Option<u32> {
        let square_options: &Vec<u32> = &self.grid[r][c].options;
        'option_loop: for option in square_options {
            // Check every other square in the row to determine if the option
            // is only applicable in this square
            for i in 0..9usize {
                // Do not check the square in question or filled squares
                let check_square: &SudokuSquare = &self.grid[i][c];
                if i != r {
                    // Skip this outer loop option if this square also has the option or the value
                    if check_square.value == Some(*option) || 
                       (check_square.value.is_none() && check_square.options.contains(option)) {
                        continue 'option_loop;
                    }
                }
            }
            // If you get here, no other square has this option. Return it.
            println!("Only row option: {} in space R={}, C={}", *option, r + 1, c + 1);
            return Some(*option);
        }
        None
    }

    // Determine if the square is the only one in its column with a certain option
    fn col_only_option(&self, r: usize, c: usize) -> Option<u32> {
        let square_options: &Vec<u32> = &self.grid[r][c].options;
        'option_loop: for option in square_options {
            // Check every other square in the column to determine if the option
            // is only applicable in this square
            for j in 0..9usize {
                // Do not check the square in question or filled squares
                let check_square: &SudokuSquare = &self.grid[r][j];
                if j != c {
                    // Skip this outer loop option if this square also has the option or the value
                    if check_square.value == Some(*option) || 
                       (check_square.value.is_none() && check_square.options.contains(option)) {
                        continue 'option_loop;
                 }
                }
            }
            // If you get here, no other square has this option. Return it.
            println!("Only col option: {} in space R={}, C={}", *option, r + 1, c + 1);
            return Some(*option);
        }
        None
    }

    // Determine if the square is the only one in its 3x3 box with a certain option
    fn box_only_option(&self, r: usize, c: usize) -> Option<u32> {
        let square_options: &Vec<u32> = &self.grid[r][c].options;
        'option_loop: for option in square_options {
            // Check every other square in the box to determine if the option
            // is only applicable in this square
            let r_start = r - r % 3;
            let c_start = c - c % 3;
            for i in r_start..r_start + 3 {
                for j in c_start..c_start + 3 {
                    let check_square: &SudokuSquare = &self.grid[i][j];
                    if i != r && j != c {
                        // Skip this outer loop option if this square also has the option or the value
                        if check_square.value == Some(*option) || 
                           (check_square.value.is_none() && check_square.options.contains(option)) {
                            continue 'option_loop;
                        }
                    }
                }
            }
            // If you get here, no other square has this option. Return it.
            println!("Only box option: {} in space R={}, C={}", *option, r + 1, c + 1);
            return Some(*option);
        }
        None
    }

    // Check the row, column, and box to determine if the square has an only option, and set value if found
    fn update_only_options(&mut self, r: usize, c: usize) -> bool {
        // Row only option check
        match self.row_only_option(r, c) {
            None => { },
            Some(i) => {
                self.set_square(r, c, i, false);
                return true;
            }
        }
        // Column only option check
        match self.col_only_option(r, c) {
            None => { },
            Some(i) => {
                self.set_square(r, c, i, false);
                return true;
            }
        }
        // Box only option check
        match self.box_only_option(r, c) {
            None => { },
            Some(i) => {
                self.set_square(r, c, i, false);
                return true;
            }
        }
        return false;
    }

    // Check the Sudoku puzzle to see if it is successfully solved
    pub fn check_solved(&mut self) -> bool {
        for i in 0..9usize {
            if !self.is_row_solved(i) {
                self.solved = false;
                return false;
            }
            if !self.is_col_solved(i) {
                self.solved = false;
                return false;
            }
            if !self.is_box_solved(i) {
                self.solved = false;
                return false;
            }
        }

        self.solved = true;
        self.status = String::default();
        return true;
    }

    fn is_row_solved(&self, r: usize) -> bool {
        // Check if each value 1-9 is present in the row
        'value_loop: for val in 1..=9u32 {
            // Check each square in the row for the value
            for i in 0..9usize {
                if self.grid[r][i].value == Some(val) {
                    continue 'value_loop;
                }
            }
            return false;
        }
        return true;
    }

    fn is_col_solved(&self, c: usize) -> bool {
        // Check if each value 1-9 is present in the column
        'value_loop: for val in 1..=9u32 {
            // Check each square in the column for the value
            for i in 0..9usize {
                if self.grid[i][c].value == Some(val) {
                    continue 'value_loop;
                }
            }
            return false;
        }
        return true;
    }

    fn is_box_solved(&self, b: usize) -> bool {
        // Check if each value 1-9 is present in the column
        'value_loop: for val in 1..=9u32 {
            // Check each square in the 3x3 box for the value
            for r_offset in 0..3usize {
                for c_offset in 0..3usize {
                    // Parse 1D 1-9 box number into 2D box index
                    let r: usize = (b / 3) * 3 + r_offset;
                    let c: usize = (b % 3) * 3 + c_offset;
                    if self.grid[r][c].value == Some(val) {
                        continue 'value_loop;
                    }
                }
            }
            return false;
        }
        return true;
    }
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