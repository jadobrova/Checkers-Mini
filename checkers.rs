// checkers.rs
use std::io::{self, Write};

const EMPTY: i8 = 0;
const WHITE: i8 = 1;
const BLACK: i8 = 2;

struct Checkers {
    board: [[i8; 8]; 8],
    turn: i8,
}

impl Checkers {
    fn new() -> Self {
        let mut board = [[EMPTY; 8]; 8];
        for row in 0..8 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    if row < 3 {
                        board[row][col] = WHITE;
                    } else if row > 4 {
                        board[row][col] = BLACK;
                    }
                }
            }
        }
        Checkers { board, turn: WHITE }
    }

    fn print_board(&self) {
        println!("  a b c d e f g h");
        for row in 0..8 {
            print!("{} ", row + 1);
            for col in 0..8 {
                let ch = match self.board[row][col] {
                    EMPTY => '·',
                    WHITE => '○',
                    BLACK => '●',
                    _ => '?',
                };
                print!("{} ", ch);
            }
            println!();
        }
    }

    fn is_valid_pos(row: i32, col: i32) -> bool {
        row >= 0 && row < 8 && col >= 0 && col < 8
    }

    fn get_moves(&self, row: usize, col: usize) -> Vec<((usize, usize), (usize, usize), Option<(usize, usize)>)> {
        let piece = self.board[row][col];
        if piece == EMPTY {
            return vec![];
        }
        let dir: i32 = if piece == WHITE { 1 } else { -1 };
        let mut moves = Vec::new();
        let row_i = row as i32;
        let col_i = col as i32;
        // простые ходы
        for dc in &[-1, 1] {
            let nr = row_i + dir;
            let nc = col_i + dc;
            if Self::is_valid_pos(nr, nc) {
                let nr_u = nr as usize;
                let nc_u = nc as usize;
                if self.board[nr_u][nc_u] == EMPTY {
                    moves.push(((row, col), (nr_u, nc_u), None));
                }
            }
        }
        // захваты
        for dc in &[-1, 1] {
            let nr = row_i + dir * 2;
            let nc = col_i + dc * 2;
            if Self::is_valid_pos(nr, nc) {
                let mr = row_i + dir;
                let mc = col_i + dc;
                let nr_u = nr as usize;
                let nc_u = nc as usize;
                let mr_u = mr as usize;
                let mc_u = mc as usize;
                if self.board[mr_u][mc_u] != EMPTY && self.board[mr_u][mc_u] != piece && self.board[nr_u][nc_u] == EMPTY {
                    moves.push(((row, col), (nr_u, nc_u), Some((mr_u, mc_u))));
                }
            }
        }
        moves
    }

    fn get_all_moves(&self, color: i8) -> Vec<((usize, usize), (usize, usize), Option<(usize, usize)>)> {
        let mut all = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if self.board[row][col] == color {
                    all.extend(self.get_moves(row, col));
                }
            }
        }
        all
    }

    fn has_captures(&self, color: i8) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if self.board[row][col] == color {
                    for m in self.get_moves(row, col) {
                        if m.2.is_some() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn make_move(&mut self, from: (usize, usize), to: (usize, usize), capture: Option<(usize, usize)>) {
        let (fr, fc) = from;
        let (tr, tc) = to;
        self.board[tr][tc] = self.board[fr][fc];
        self.board[fr][fc] = EMPTY;
        if let Some((cr, cc)) = capture {
            self.board[cr][cc] = EMPTY;
        }
    }

    fn is_game_over(&self) -> bool {
        let white_moves = self.get_all_moves(WHITE);
        let black_moves = self.get_all_moves(BLACK);
        if white_moves.is_empty() || black_moves.is_empty() {
            return true;
        }
        let mut has_white = false;
        let mut has_black = false;
        for row in 0..8 {
            for col in 0..8 {
                match self.board[row][col] {
                    WHITE => has_white = true,
                    BLACK => has_black = true,
                    _ => {}
                }
            }
        }
        !has_white || !has_black
    }

    fn get_winner(&self) -> i8 {
        let white_moves = self.get_all_moves(WHITE);
        let black_moves = self.get_all_moves(BLACK);
        let mut has_white = false;
        let mut has_black = false;
        for row in 0..8 {
            for col in 0..8 {
                match self.board[row][col] {
                    WHITE => has_white = true,
                    BLACK => has_black = true,
                    _ => {}
                }
            }
        }
        if !has_white || white_moves.is_empty() {
            return BLACK;
        }
        if !has_black || black_moves.is_empty() {
            return WHITE;
        }
        0
    }

    fn parse_move(s: &str) -> Option<((usize, usize), (usize, usize))> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }
        if parts[0].len() < 2 || parts[1].len() < 2 {
            return None;
        }
        let c1 = parts[0].chars().nth(0).unwrap();
        let r1 = parts[0].chars().nth(1).unwrap();
        let c2 = parts[1].chars().nth(0).unwrap();
        let r2 = parts[1].chars().nth(1).unwrap();
        let col1 = (c1 as u8 - b'a') as i32;
        let row1 = (r1 as u8 - b'1') as i32;
        let col2 = (c2 as u8 - b'a') as i32;
        let row2 = (r2 as u8 - b'1') as i32;
        if row1 < 0 || row1 > 7 || col1 < 0 || col1 > 7 || row2 < 0 || row2 > 7 || col2 < 0 || col2 > 7 {
            return None;
        }
        Some(((row1 as usize, col1 as usize), (row2 as usize, col2 as usize)))
    }

    fn play(&mut self) {
        println!("Добро пожаловать в мини-шашки!");
        println!("Белые (○) ходят первыми.");
        println!("Вводите ход в формате: a2 b3");
        let stdin = io::stdin();
        let mut input = String::new();
        while !self.is_game_over() {
            self.print_board();
            let color = self.turn;
            let color_name = if color == WHITE { "белых (○)" } else { "чёрных (●)" };
            println!("Ход {}.", color_name);
            let all_moves = self.get_all_moves(color);
            if all_moves.is_empty() {
                println!("Нет доступных ходов, игра окончена.");
                break;
            }
            let has_cap = self.has_captures(color);
            let moves = if has_cap {
                println!("Обязательный захват!");
                all_moves.into_iter().filter(|m| m.2.is_some()).collect()
            } else {
                all_moves
            };
            loop {
                print!("> ");
                io::stdout().flush().unwrap();
                input.clear();
                stdin.read_line(&mut input).unwrap();
                let cmd = input.trim();
                if cmd == "quit" {
                    return;
                }
                if let Some((from, to)) = Self::parse_move(cmd) {
                    let found = moves.iter().position(|m| m.0 == from && m.1 == to);
                    if let Some(idx) = found {
                        let m = &moves[idx];
                        self.make_move(m.0, m.1, m.2);
                        self.turn = if color == WHITE { BLACK } else { WHITE };
                        break;
                    } else {
                        println!("Неверный ход. Попробуйте снова.");
                    }
                } else {
                    println!("Неверный формат. Используйте: a2 b3");
                }
            }
        }
        self.print_board();
        let winner = self.get_winner();
        if winner == WHITE {
            println!("Победили белые (○)!");
        } else if winner == BLACK {
            println!("Победили чёрные (●)!");
        } else {
            println!("Ничья?");
        }
    }
}

fn main() {
    let mut game = Checkers::new();
    game.play();
}
