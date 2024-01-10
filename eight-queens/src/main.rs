const N: usize = 8;

fn is_safe(chess_board: &[[bool; N]; N], rank: usize, file: usize) -> bool {
    for i in 0..=file {
        if chess_board[rank][i] {
            return false;
        }
    }

    let range1 = (0..=rank).rev();
    let range2 = (0..=file).rev();

    for (i, j) in range1.zip(range2) {
        if chess_board[i][j] {
            return false;
        }
    }

    let range1 = rank..=N-1;
    let range2 = (0..=file).rev();

    for (i, j) in range1.zip(range2) {
        if chess_board[i][j] {
            return false;
        }
    }

    true
}

fn solve_n_queens(chess_board: &mut [[bool; N]; N], col: usize) -> bool {
    if col as usize == N {
        for rank in 0..N {
            for file in 0..N {
                print!("{} ", if chess_board[rank][file] { 'Q' } else { '-' });
            }
            println!()
        }
        return true;
    }

    for i in 0..N {
        if is_safe(chess_board, i, col) {
            chess_board[i][col] = true;
            if solve_n_queens(chess_board, col + 1) {
                return true;
            }
            chess_board[i][col] = false;
        }
    }
    false
}

fn main() {
    let mut chess_board: [[bool; N]; N] = [[false; N]; N];

    if !solve_n_queens(&mut chess_board, 0) {
        println!("No solution found");
    }
}
