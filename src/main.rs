use std::cell::*;
use std::rc::Rc;
use std::{thread, time};
use pancurses::*;

mod display;
mod logic;


fn main(){
    if has_colors(){
        start_color();
    }
    noecho();
    
    let win =initscr();
    win.nodelay(true); 
    let delay = time::Duration::from_millis(300);

    let eng = ['!', '?', '@'];
    let blank = [' '];
    let scrn = logic::gen_screen(&eng.to_vec(), &(win.get_max_y() as usize), &(win.get_max_x() as usize));
    
    display::disp_non(&win); 
    thread::sleep(delay);
    win.keypad(true);
    while win.getch() != Some(Input::Character('c')){  
        let conv = display::to_charcolor(scrn.clone());
        display::display(&win, conv, COLOR_YELLOW);
    }
    endwin();
}

