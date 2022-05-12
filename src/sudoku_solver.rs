use actix_web::{get, web, Responder};
use sudoku::Sudoku;

// Sudokus can be created from &str's in both block or line formats or directly from bytes.
// here, an example in line format
// let sudoku_line = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";
fn solve(sudoku_line: String) -> Option<String> {

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