use async_recursion::async_recursion;
use macroquad::prelude::*;

const WIDTH: f32 = 900.0;
const HEIGHT: f32 = 900.0;
const TEXT_SIZE: f32 = 100.0;

struct Sudoku {
    board: [[u8; 9]; 9],
    solved: [[u8; 9]; 9],
}

impl Sudoku {
    fn new(board: [[u8; 9]; 9]) -> Self {
        Self {
            board,
            solved: board,
        }
    }

    fn is_possible(&self, row: usize, col: usize, n: u8) -> bool {
        for i in 0..9 {
            if (self.solved[i][col] != 0 && self.solved[i][col] == n)
                || (self.solved[row][i] != 0 && self.solved[row][i] == n)
                || (self.solved[3 * (row / 3) + i / 3][3 * (col / 3) + i % 3] != 0
                    && self.solved[3 * (row / 3) + i / 3][3 * (col / 3) + i % 3] == n)
            {
                return false;
            }
        }

        true
    }

    fn draw(&self) {
        // boarder
        for i in 0..7 {
            for j in 0..7 {
                let x = (WIDTH / 3.0) * j as f32;
                let y = (HEIGHT / 3.0) * i as f32;
                let w = WIDTH / 3.0;
                let h = HEIGHT / 3.0;

                draw_rectangle_lines(x / 3.0, y, w, h, 5.0, DARKGRAY);
                draw_rectangle_lines(x, y / 3.0, w, h, 5.0, DARKGRAY);
                draw_rectangle_lines(x, y, w, h, 10.0, GRAY);
            }
        }

        // numbers
        for i in 0..9 {
            for j in 0..9 {
                let x = (WIDTH / 3.0) * (j as f32 / 3.0);
                let y = (HEIGHT / 3.0) * (i as f32 / 3.0);
                let w = WIDTH / 3.0;
                let h = HEIGHT / 3.0;
                let board = self.board[i][j];
                let solved = self.solved[i][j];
                let text = if board != 0 {
                    format!("{}", board)
                } else {
                    format!("{}", solved)
                };
                let text_size = measure_text(&text, None, TEXT_SIZE as u16, 1.0);
                let text_x = (x + w / 3.0) - w / 6.0 - text_size.width / 2.0;
                let text_y = (y + h / 3.0) - h / 6.0 + text_size.height / 2.0;

                if board != 0 {
                    draw_text(&text, text_x, text_y, TEXT_SIZE, LIGHTGRAY);
                } else if solved != 0 {
                    draw_text(&text, text_x, text_y, TEXT_SIZE, GRAY);
                }
            }
        }
    }

    #[async_recursion]
    async fn solve(&mut self) -> bool {
        self.draw();
        next_frame().await;
        for i in 0..9 {
            for j in 0..9 {
                if self.solved[i][j] == 0 {
                    for n in 1..=9 {
                        if self.is_possible(i, j, n) {
                            self.solved[i][j] = n;

                            if self.solve().await {
                                return true;
                            } else {
                                self.solved[i][j] = 0;
                            }
                        }
                    }

                    return false;
                }
            }
        }

        true
    }
}

#[macroquad::main(conf)]
async fn main() {
    let board = [
        [0, 4, 8, 2, 0, 0, 0, 0, 1],
        [1, 0, 0, 3, 8, 4, 7, 2, 6],
        [3, 0, 0, 7, 0, 1, 9, 4, 8],
        [0, 7, 2, 6, 4, 5, 1, 8, 0],
        [8, 0, 0, 0, 0, 2, 4, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 7],
        [0, 8, 4, 0, 0, 0, 3, 0, 0],
        [6, 0, 0, 4, 1, 0, 0, 0, 2],
        [0, 0, 3, 0, 0, 0, 0, 7, 4],
    ];

    let mut sudoku = Sudoku::new(board);

    loop {
        sudoku.solve().await;
        sudoku.draw();
        next_frame().await
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "Sudoku".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}
