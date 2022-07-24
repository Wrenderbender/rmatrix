use std::fmt::Display;

mod display;
mod logic;

fn main() {
    let eng = ['!', '?', '@'];
    let blank = [' '];
    let scrn = logic::gen_screen(&eng.to_vec(), 15, 15);
    let conv = display::to_charcolor(scrn.clone());
    println!("{:?}", &scrn);
    println!("{:?}", conv);
    //let col = logic::gen_column(&blank.to_vec(), scrn[0].len());
    //let cycle = logic::next_line(scrn[0].clone(), col[0], (scrn.len() -1) as i32);
}

fn pretty_print<T: Display>(x: &Vec<Vec<T>>) {
    for i in x {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}
