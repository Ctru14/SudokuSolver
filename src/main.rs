use iced::widget::{button, column, container, text, text_input};
use iced::widget::{Column, Container, Row, Text};
use iced::{alignment, border};
use iced::{Application, Border, Color, Element, Settings, Theme};
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
        let self_grid_widget: Container<'_, Message> = create_grid_widget(&self.grid);

        column![
            text("Welcome to the Sudoku Solver!").size(30),
            self_grid_widget,
        ]
    }

    fn square_text_update(&mut self, id: &str, input: &str) {
        // Parse ID into row and column indices
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
fn create_grid_widget(grid: &Vec<Vec<SudokuSquare>>) -> Container<'static, Message, Theme> {
    // Create a column widget to hold the rows
    let mut column = Column::new();

    // Create a 3x3 Bordered Container grid of 3x3 Sudoku squares
    for outer_c in 0..3 {
        let mut outer_row = Row::new();
        for outer_r in 0..3 {
            let mut inner_column = Column::new();
            for inner_c in 0..3 {
                let mut inner_row = Row::new();
                for inner_r in 0..3 {
                    // Create proper row and column indices
                    let c: usize = outer_c * 3 + inner_c;
                    let r: usize = outer_r * 3 + inner_r;

                    // Bounds check on grid indices
                    if grid.len() <= c || grid[c].len() <= r {
                        println!("Error: Grid indices out of bounds: C{} R{}", c, r);
                        return Container::new(Text::new("Error: Grid indices out of bounds"));
                    }

                    // Create the Sudoku cell with a text box for user input
                    // Give each text box an ID with its indices to identify which text is updated
                    let box_id: String = format!("square-C{}-R{}", c, r);
                    let text_value = match grid[c][r].value {
                        Some(value) => value.to_string(),
                        None => "".to_string(),
                    };

                    let input_square = text_input("", &text_value.to_string())
                        .on_input(move |new_text| {
                            Message::TextChanged(box_id.to_string(), new_text)
                        })
                        .padding(5)
                        .size(25)
                        .width(50)
                        .align_x(alignment::Horizontal::Center);

                    inner_row = inner_row.push(input_square);
                }
                inner_column = inner_column.push(inner_row);
            }
            // Wrap the 3x3 Square in a Container with a border
            let bordered_3x3_square = Container::new(inner_column)
                .style(container::bordered_box)
                .padding(2);

            outer_row = outer_row.push(bordered_3x3_square);
        }

        column = column.push(outer_row);
    }

    // Add one more thick border around the whole Sudoku grid
    let bordered_grid: Container<'_, Message, Theme, _> = Container::new(column)
        .style(container::bordered_box)
        .padding(2);

    return bordered_grid;
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
