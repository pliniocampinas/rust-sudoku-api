use actix_web::{get, web, Responder};

fn str_to_board_mut(s: &str, board: &mut [[u8; 9]; 9]) {
    let chars: Vec<char> = s
        .chars()
        .filter_map(|c| {
            if c == '.' {
                Some('0')
            } else {
                Some(c)
            }
        })
        .collect();
    for (i, chunk) in chars.chunks(9).enumerate() {
        for (j, c) in chunk.iter().enumerate() {
            board[i][j] = c.to_digit(10).unwrap() as u8;
        }
    }
}

fn solve_sudoku(grid: &mut [[u8; 9]; 9]) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j] == 0 {
                for k in 1..10 {
                    if is_valid(grid, i, j, k) {
                        grid[i][j] = k;
                        if solve_sudoku(grid) {
                            return true;
                        } else {
                            grid[i][j] = 0;
                        }
                    }
                }
                return false;
            }
        }
    }
    true
}

fn is_valid(grid: &[[u8; 9]; 9], row: usize, col: usize, k: u8) -> bool {
    // check if k is valid in the row
    for j in 0..9 {
        if grid[row][j] == k {
            return false;
        }
    }

    // check if k is valid in the column
    for i in 0..9 {
        if grid[i][col] == k {
            return false;
        }
    }

    // check if k is valid in the subgrid
    let subgrid_row = (row / 3) * 3;
    let subgrid_col = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if grid[subgrid_row + i][subgrid_col + j] == k {
                return false;
            }
        }
    }

    true
}

// Sudokus can be created from &str's in both block or line formats or directly from bytes.
// here, an example in line format
// let sudoku_line = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";
fn solve(sudoku_line: String) -> Option<String> {
    if sudoku_line.len() != 81 {
        return None;
    }

    let mut board = [[0; 9]; 9];
    str_to_board_mut(sudoku_line.as_str(), &mut board);
    let solved = solve_sudoku(&mut board);

    if !solved {
        return None;
    }

    let mut solved_sudoku_line = String::new();

    for line in board {
        for cell in line {
            solved_sudoku_line += &cell.to_string();
        }
    }

    return Some(format!("{solved_sudoku_line}"));
}

#[get("/solve/{sudoku_line}")]
pub async fn handle(sudoku_line: web::Path<String>) -> impl Responder {

    if let Some(solution) = solve(sudoku_line.into_inner()) {
        return format!("{solution}");
    }

    format!("Invalid Sudoku")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_board_mut_with_dots() {
        let s = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";
        let mut board = [[0; 9]; 9];
        str_to_board_mut(s, &mut board);
        assert_eq!(board, [
            [0,0,0,2,0,0,0,6,3],
            [3,0,0,0,0,5,4,0,1],
            [0,0,1,0,0,3,9,8,0],
            [0,0,0,0,0,0,0,9,0],
            [0,0,0,5,3,8,0,0,0],
            [0,3,0,0,0,0,0,0,0],
            [0,2,6,3,0,0,5,0,0],
            [5,0,3,7,0,0,0,0,8],
            [4,7,0,0,0,1,0,0,0],
        ])
    }

    #[test]
    fn test_valid_solution() {
        let mut grid = [        
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let result = solve_sudoku(&mut grid);
        assert_eq!(result, true);

        let mut grid = [        
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [4, 5, 6, 7, 8, 9, 1, 2, 3],
            [7, 8, 9, 1, 2, 3, 4, 5, 6],
            [2, 1, 4, 3, 6, 5, 8, 9, 7],
            [3, 6, 5, 8, 9, 7, 2, 1, 4],
            [8, 9, 7, 2, 1, 4, 3, 6, 5],
            [5, 3, 1, 6, 4, 2, 9, 7, 8],
            [6, 4, 2, 9, 7, 8, 5, 3, 1],
            [9, 7, 8, 5, 3, 1, 6, 4, 2],
        ];
        let result = solve_sudoku(&mut grid);
        assert_eq!(result, true);
    }

    #[test]
    fn test_grid_solution() {
        let s = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";
        let mut board = [[0; 9]; 9];
        str_to_board_mut(s, &mut board);
        solve_sudoku(&mut board);
        assert_eq!(board, [
            [8, 5, 4, 2, 1, 9, 7, 6, 3], 
            [3, 9, 7, 8, 6, 5, 4, 2, 1], 
            [2, 6, 1, 4, 7, 3, 9, 8, 5], 
            [7, 8, 5, 1, 2, 6, 3, 9, 4], 
            [6, 4, 9, 5, 3, 8, 1, 7, 2], 
            [1, 3, 2, 9, 4, 7, 8, 5, 6], 
            [9, 2, 6, 3, 8, 4, 5, 1, 7], 
            [5, 1, 3, 7, 9, 2, 6, 4, 8], 
            [4, 7, 8, 6, 5, 1, 2, 3, 9]
        ]);
    }
}