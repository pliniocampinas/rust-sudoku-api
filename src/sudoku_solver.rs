use actix_web::{get, web, Responder};
use sudoku::Sudoku;

// Sudokus can be created from &str's in both block or line formats or directly from bytes.
// here, an example in line format
// let sudoku_line = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";

struct SudokuBoard {
    sudoku_line: [char; 81]
}

impl SudokuBoard {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn getCell(&self, x: usize, y: usize) -> char {
        let index = x + y*9;
        return self.sudoku_line[index];
    }

    // Another associated function, taking two arguments:
    // fn new(x: f64, y: f64) -> Point {
    //     Point { x: x, y: y }
    // }
}

// TODO
fn init_board(sudoku_board: &SudokuBoard, sudoku_line: String) {
    let mut split = sudoku_line.split("");
    for s in split {
        println!("{}", s)
    }
}

fn is_a_possible_move(sudoku_board: &SudokuBoard, x: usize, y: usize, value: char) -> bool {
    // Test for lines and column matches
    for i in 0..81 {
        if sudoku_board.getCell(i, y) == value {
            return false
        }
        if sudoku_board.getCell(x, i) == value {
            return false
        }
    }


    // Test 3x3 squares
    let x0 = (x/3) * 3;
    let y0 = (y/3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if sudoku_board.getCell(x0+j, y0+i) == value {
                return false
            }
        }
    }

    true
}

// TODO
fn solve_unique(sudoku_board: &SudokuBoard) -> Option<String> {
    None
}

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