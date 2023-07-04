use std::{thread, time};
extern crate rand;

const BOARD_HEIGHT: usize = 40;
const BOARD_WIDTH: usize = BOARD_HEIGHT * 2;

type Row = [bool; BOARD_WIDTH];
type Board = [Row; BOARD_HEIGHT];

fn get_count(row: usize, col: usize, board: &[[bool; BOARD_WIDTH]; BOARD_HEIGHT]) -> u8 {
    // get top right bottom left 
    let mut count: u8 = 0;

    // top
    if row > 0 && board[row - 1][col] {
        count += 1;
    }

    // top right
    if row > 0 && col < BOARD_WIDTH - 1 && board[row - 1][col + 1] {
        count += 1;
    }

    // right 
    if col < BOARD_WIDTH - 1 && board[row][col + 1] {
        count += 1;
    }

    // bottom right 
    if row < BOARD_HEIGHT - 1 && col < BOARD_WIDTH - 1 && board[row + 1][col + 1] {
        count += 1;
    }

    // bottom 
    if row < BOARD_HEIGHT - 1 && board[row + 1][col] { 
        count += 1;
    }

    // bottom left
    if row < BOARD_HEIGHT - 1 && col > 0 && board[row + 1][col - 1] {
        count += 1;
    }

    // left
    if col > 0 && board[row][col -1] {
        count += 1;
    }

    // top left
    if row > 0 && col > 0 && board[row - 1][col - 1] {
        count += 1;
    }
    
    return count;
}

fn has_two_or_three_neighbours(row: usize, col: usize, board: &[[bool; BOARD_WIDTH]; BOARD_HEIGHT]) -> bool {
    let count = get_count(row, col, board);
    return count == 2 || count == 3;
}

fn has_three_neighbours(row: usize, col: usize, board: &[[bool; BOARD_WIDTH]; BOARD_HEIGHT]) -> bool {
    let count = get_count(row, col, board);
    return count == 3;
}

fn seed_rand_board(board: &mut [[bool; BOARD_WIDTH]; BOARD_HEIGHT]) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            board[i][j] = rand::random::<bool>();
        }
    }
}

/*
 * Ehh it's life...here are the rules 
 *
 * 1. Any live cell with two or three live neighbours survives.
 * 2. Any dead cell with three live neighbours becomes a live cell.
 * 3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
 */
fn main() {
    // get boards
    let mut board: Board = [[false; BOARD_WIDTH]; BOARD_HEIGHT];

    seed_rand_board(&mut board);

    let mut next_gen: Board = board.clone();

    let delay = time::Duration::from_millis(250);

    // run forever
    loop {
        // update board 
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                // print current cell
                if board[i][j] {
                    print!("■");
                } else {
                    print!(" ");
                }

                // calculate next gen cell value

                // 1. Any live cell with two or three live neighbours survives.
                if board[i][j] && has_two_or_three_neighbours(i, j, &board) {
                    continue;
                }

                // 2. Any dead cell with three live neighbours becomes a live cell.
                if !board[i][j] && has_three_neighbours(i, j, &board) {
                    next_gen[i][j] = true;
                    continue;
                }

                // 3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
                if board[i][j] {
                    next_gen[i][j] = false;
                }
            }

            print!("\n");
        }

        // clear terminal 
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // update board 
        board = next_gen.clone();
        thread::sleep(delay);
    }
}
