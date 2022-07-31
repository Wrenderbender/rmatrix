use pancurses::*;
use std::rc::*;

#[derive(Debug)]
pub enum CharColor<T> {
    Standard(T),
    Dim(T),
}


pub fn disp_non(win:&Window){
    win.addch(' ');
    win.refresh();
}

pub fn display(win: &Window, scrn: Vec<Vec<CharColor<char>>>, color:i16){
    init_pair(1, color, pancurses::COLOR_BLACK);
    for (i, val) in scrn.iter().enumerate() {
        for (j, val2) in val.iter().enumerate() {
            match *val2 {
                CharColor::Standard(t) => {
                    win.attrset(COLOR_PAIR(1));
                    win.mvaddch(j as i32, i as i32, t)},
                CharColor::Dim(t) => {
                    win.attrset(pancurses::A_DIM);
                    win.mvaddch(j as i32, i as i32, t)
                }
            };
        }
    }
    win.refresh();
}

pub fn to_charcolor(screen: Vec<Vec<char>>) -> Vec<Vec<CharColor<char>>> {
    let mut new_screen: Vec<Vec<CharColor<char>>> = Vec::new();
    for i in screen {
        let mut column: Vec<CharColor<char>> = Vec::new();
        let mut iter = i.iter().peekable();
        let mut next = iter.next();
        while next != None {
            if next.unwrap() == &' ' {
                if iter.peek().unwrap().is_alphabetic() {
                    column.push(CharColor::Dim(**iter.peek().unwrap()));
                } else {
                    column.push(CharColor::Standard(*next.unwrap()));
                }
            } else {
                column.push(CharColor::Standard(*next.unwrap()));
            }
            next = iter.next();
        }
        new_screen.push(column);
    }
    new_screen
}
