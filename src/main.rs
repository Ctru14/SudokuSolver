use iced::widget::{button, column, text, text_input};
use iced::widget::{Column, Container, Row, Text};
use iced::alignment;
use iced::{Application, Element, Settings, Theme};
use regex::Regex;

pub fn main() -> iced::Result {
    iced::application("Sudoku Solver", Sudoku::update, Sudoku::view)
        .theme(|_| Theme::Dark)
        .centered()
        .run()
}

#[derive(Default, Debug, Clone)]
struct SudokuSquare {
    value: Option<u32>,
    candidates: Vec<u32>,
    given: bool,
}

#[derive(Debug, Clone)]
struct Sudoku {
    value: usize,
    text: String,
    grid: Vec<Vec<SudokuSquare>>,
}

impl Default for Sudoku {
    fn default() -> Self {
        Sudoku {
            value: 0,
            text: String::from("Hello"),
            grid: vec![vec![SudokuSquare::default(); 9]; 9],
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TextChanged(String, String), // (ID, Text)
}

impl Sudoku {
    fn update(&mut self, message: Message) {
        match message {
            Message::TextChanged(id, input) => {
                self.square_text_update(&id, &input);
            }
        }
    }

    fn view(&self) -> Column<Message> {
        // Create widget from Sudoku grid
        let self_grid_widget: Column<'_, Message> = create_grid_widget(&self.grid);

        column![
            text("Welcome to the Sudoku Solver!").size(30),
            self_grid_widget,
        ]
    }

    fn square_text_update(&mut self, id: &str, input: &str) {
        // Parse ID into row and column indices
        let re = Regex::new(r"square-C(\d+)-R(\d+)").unwrap();
        let c: usize;
        let r: usize;
        (c, r) = match parse_square_id(&id) {
            Some((c, r)) => (c, r),
            None => return,
        };

        // Get current square value
        let prev_text = match self.grid[c][r].value {
            Some(value) => value.to_string(),
            None => "".to_string(),
        };

        // Update square value with new input
        let new_text = get_new_square_text(&prev_text, &input);
        if new_text.len() > 0 {
            self.grid[c][r].value = Some(new_text.parse::<u32>().unwrap());
        } else {
            self.grid[c][r].value = None;
        }
    }
}

// ---------------------------- Helper functions ----------------------------

// Convers Sudoku grid to Iced Column widget
fn create_grid_widget(grid: &Vec<Vec<SudokuSquare>>) -> Column<'static, Message> {
    // Create a column widget to hold the rows
    let mut column = Column::new();

    // Add individual rows
    for c in 0..grid.len() {
        // row in grid {
        let mut row_widget = Row::new();
        for r in 0..grid[c].len() {
            // Create the Sudoku cell with a text box for user input
            // Give each text box an ID with its indices to identify which text is updated
            let box_id: String = format!("square-C{}-R{}", c, r);
            let text_value = match grid[c][r].value {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };
            row_widget = row_widget.push(
                text_input("", &text_value.to_string())
                    .on_input(move |new_text| Message::TextChanged(box_id.to_string(), new_text))
                    .padding(5)
                    .size(25)
                    .width(50)
                    .align_x(alignment::Horizontal::Center)
            );
        }
        column = column.push(row_widget);
    }

    return column;
}

// Check the Sudoku square text input to display the right character
fn get_new_square_text(prev: &str, input: &str) -> String {
    // Filter the input to only allow Sudoku digits 1-9
    let filter_text: Vec<char> = input
        .chars()
        .filter(|c| c.is_digit(10) && *c != '0')
        .collect();

    // Return empty string if no digits found
    if filter_text.len() == 0 {
        return "".to_string();
    }

    // Check input to set text to the newly typed digit
    // Since a new digit can be either at the beginning or the end we can't always take the last digit
    if filter_text.len() == 2 && prev.len() == 1 {
        if filter_text[0] == prev.chars().next().unwrap() {
            return filter_text[1].to_string();
        } else {
            return filter_text[0].to_string();
        }
    }

    // Default to returning the last char if the check doesn't hit
    return filter_text[filter_text.len() - 1].to_string();
}

// Parse the text box Sudoku square ID to get the row and column indices
fn parse_square_id(id: &str) -> Option<(usize, usize)> {
    let re = Regex::new(r"square-C(\d+)-R(\d+)").unwrap();
    let c: usize;
    let r: usize;
    if let Some(captures) = re.captures(&id) {
        c = captures
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        r = captures
            .get(2)
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        return Some((c, r));
    } else {
        println!("Error parsing ID for indices: {}", id);
        return None;
    }
}
