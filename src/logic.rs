use rand::{self, Rng};
use std::{collections::VecDeque, process};

pub enum State {
    Tick,
    Quit,
}

pub fn apply_state(cur_state: State) {
    match cur_state {
        State::Quit => std::process::exit(0),
        State::Tick => (), /*do stuff*/
    }
}

pub fn gen_column(charset: &Vec<char>, height: usize) -> Vec<char> {
    let mut col: Vec<char> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..height {
        col.push(charset[rng.gen_range(0..charset.len())]);
    }
    col
}

pub fn next_line(column: Vec<char>, next_char: char, height: i32) -> VecDeque<char> {
    let mut column_q = VecDeque::from(column);
    if height == column_q.len() as i32 - 1 {
        column_q.push_front(next_char);
        column_q.pop_back();
        return column_q;
    }
    column_q.push_front(next_char);
    column_q
}

pub fn gen_screen(charset: &Vec<char>, height: &usize, width: &usize) -> Vec<Vec<char>> {
    let mut scrn: Vec<Vec<char>> = Vec::new();
    for _ in 0..*width {
        scrn.push(gen_column(charset, *height));
    }
    scrn
}
