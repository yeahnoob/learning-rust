extern crate ncurses;

use std::io;
use std::convert::AsRef;
use ncurses::*;

const CONFIRM_STRING: &'static str = "y";
const OUTPUT_EXAMPLE: &'static str =
"Great Firewall dislike VPN protocol.\nGFW 不喜欢VPN协议。";


fn ex1(s: &str) {
    initscr();
    printw(s);
    refresh();
    getch();
    endwin();
}

fn main() {
    let mylocale = LcCategory::all;

    setlocale(mylocale, "zh_CN.UTF-8");

    let mut input = String::new();
    
    println!("[ncurses-rs examples]\n");
    
    println!(" example_1. Press \"{}\" or [Enter] to run it...:", CONFIRM_STRING);
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Fail to get keyboard input");
    match input.trim().as_ref() {
        CONFIRM_STRING | "" => ex1(OUTPUT_EXAMPLE),
        _ => println!("...Go to next step.")
    }

    println!("example_2. Press [Enter] to run it...");
    // ex2();
}
