use actix_web::{get, web, Responder};
use sudoku::Sudoku;
use std::char;

// Sudokus can be created from &str's in both block or line formats or directly from bytes.
// here, an example in line format
// let sudoku_line = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";

struct SudokuBoard {
    sudoku_line: [char; 81]
}

impl SudokuBoard {
    fn get_cell(&self, x: usize, y: usize) -> char {
        let index = x + y*9;
        return self.sudoku_line[index];
    }

    fn set_cell(&mut self, x: usize, y: usize, value: char) {
        let index = x + y*9;
        self.sudoku_line[index] = value;
    }

    fn new(sudoku_line: String) -> SudokuBoard {
        let mut new_board = SudokuBoard {
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

fn is_a_possible_move(sudoku_board: &SudokuBoard, x: usize, y: usize, value: char) -> bool {
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

// fn solve_unique(sudoku_board: &SudokuBoard) -> Option<String> {
//     for x in 0..9 {
//         for y in 0..9 {
//             if sudoku_board.get_cell(x, y) == '.' {

//                 for n in 1..10 {
//                     let nChar = char::from_digit(n, 10).unwrap();

//                     if is_a_possible_move(sudoku_board, x, y, nChar) {
//                         sudoku_board.set_cell(x, y, nChar);
//                         if let Some(solution) = solve_unique(sudoku_board) {
//                             return Some(solution);
//                         }
//                         sudoku_board.set_cell(x, y, '.');
//                     }
//                 }
//                 return None
//             }
//         }
//     }

//     None
// }

fn solve(sudoku_line: String) -> Option<String> {
    if sudoku_line.len() != 81 {
        return None;
    }

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