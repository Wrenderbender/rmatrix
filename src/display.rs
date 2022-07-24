use pancurses;

#[derive(Debug)]
pub enum CharColor<T> {
    Standard(T),
    Dim(T),
}

pub fn display(win: pancurses::Window, scrn: Vec<Vec<CharColor<char>>>) -> Result<(), ()> {
    for (i, val) in scrn.iter().enumerate() {
        for (j, val2) in val.iter().enumerate() {
            match val2 {
                &CharColor::Standard(T) => win.mvaddch(j as i32, i as i32, T),
                &CharColor::Dim(T) => {
                    win.attron(pancurses::A_DIM);
                    win.mvaddch(j as i32, i as i32, T)
                }
            };
        }
    }
    Ok(())
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
