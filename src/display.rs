use pancurses;

enum CharColor<T>{
    Standard(T),
    Dim(T),
}



fn display(win:pancurses::Window, scrn:Vec<Vec<CharColor<char>>>) -> Result<(),()>{
    for (i,val) in scrn.iter().enumerate(){
        for (j,val2) in val.iter().enumerate(){
            match val2{
                &CharColor::Standard(T) =>  win.mvaddch(j as i32, i as i32, T),
                &CharColor::Dim(T) => {
                    win.attron(pancurses::A_DIM); 
                    win.mvaddch(j as i32, i as i32, T)},
            };
        }
    }
    Ok(())
}
