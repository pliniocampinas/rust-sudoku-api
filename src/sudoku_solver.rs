use actix_web::{get, web, Responder};
use sudoku::Sudoku;
use std::char;

// Sudokus can be created from &str's in both block or line formats or directly from bytes.
// here, an example in line format
// let sudoku_line = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";

struct SudokuBoardIter {
    sudoku_line: [char; 81]
}

impl SudokuBoardIter {
    fn get_cell(&self, x: usize, y: usize) -> char {
        let index = x + y*9;
        return self.sudoku_line[index];
    }

    fn set_cell(&mut self, x: usize, y: usize, value: char) {
        let index = x + y*9;
        self.sudoku_line[index] = value;
    }

    fn is_not_full(&self) -> bool {
        self.as_str().contains(".")
    }

    fn is_cell_empty(&self, x: usize, y: usize) -> bool {
        let index = x + y*9;
        return self.sudoku_line[index] == '.';
    }

    fn as_str(&self) -> String {
        let mut self_str = String::from("");

        for _ in 0..81 {
            self_str = self_str + ".";
        }

        self_str
    }

    fn new(sudoku_line: String) -> SudokuBoardIter {
        let mut new_board = SudokuBoardIter {
            sudoku_line: ['.'; 81]
        };

        let mut index = 0;
        let char_vec: Vec<char> = sudoku_line.chars().collect();
        for c in char_vec {
            println!("{}", c);
            index = index + 1;
            new_board.sudoku_line[index] = c;
        }
        return new_board;
    }
}

fn new_board(sudoku_line: String) -> SudokuBoardIter {
    let mut new_board = SudokuBoardIter {
        sudoku_line: ['.'; 81]
    };

    let mut index = 0;
    let char_vec: Vec<char> = sudoku_line.chars().collect();
    for c in char_vec {
        println!("{}", c);
        index = index + 1;
        new_board.sudoku_line[index] = c;
    }
    return new_board;
}

fn is_a_valid_move(sudoku_board: &SudokuBoardIter, x: usize, y: usize, value: char) -> bool {
    // Test for lines and column matches
    for i in 0..9 {
        if sudoku_board.get_cell(i, y) == value {
            return false
        }
        if sudoku_board.get_cell(x, i) == value {
            return false
        }
    }

    // Test 3x3 squares
    let x0 = (x/3) * 3;
    let y0 = (y/3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if sudoku_board.get_cell(x0+j, y0+i) == value {
                return false
            }
        }
    }

    true
}

fn solve_unique_iter(sudoku_board: &mut SudokuBoardIter) -> Option<String> {
    let mut cursor_stack = Vec::new();
    // Stack: x, y, n, last_n
    cursor_stack.push((0,0,0,0));


    while sudoku_board.is_not_full() {
        // Check the board from left to right and top to bottom
        let mut x = 0;
        while x < 9 {
            let mut y = 0;
            while y < 9 {
                if sudoku_board.is_cell_empty(x, y) {
                    let mut found_valid_n = false;

                    // Check numbers between 1 and 9
                    let mut n = 1;
                    while n <= 9 {
                        let last = cursor_stack.last_mut().unwrap();

                        // If last attempt was 9, break and pop solution
                        if last.3 >= 9 {
                            break;
                        }
                        // If next move was tried before, jump forward
                        if n <= last.3 {
                            n = last.3 + 1;
                        }

                        let n_char = char::from_digit(n, 10).unwrap();
                        if is_a_valid_move(sudoku_board, x, y, n_char) {
                            // Set last_n for backtracking
                            last.3 = n;
                            found_valid_n = true;
                            sudoku_board.set_cell(x, y, n_char);
                            cursor_stack.push((x, y, n, 0));
                            break;
                        }
                        n = n + 1;
                    }

                    if found_valid_n == false {
                        cursor_stack.pop();
                    }
                }
                y = y + 1;
            }
            x = x + 1;
        }
    }

    None
}

fn solve(sudoku_line: String) -> Option<String> {
    if sudoku_line.len() != 81 {
        return None;
    }

    // let mut sudoku_board = new_board(sudoku_line.clone());
    // solve_unique_iter(sudoku_board);

    let sudoku_solver = Sudoku::from_str_line(sudoku_line.as_str()).unwrap();

    if let Some(solution) = sudoku_solver.solve_unique() {
        return Some(format!("{solution}"));
    }

    None
}

#[get("/solve/{sudoku_line}")]
pub async fn handle(sudoku_line: web::Path<String>) -> impl Responder {

    if let Some(solution) = solve(sudoku_line.into_inner()) {
        return format!("{solution}");
    }

    format!("Invalid Sudoku")
}