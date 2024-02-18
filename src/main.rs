use game::Game;
use lexer::Lexer;
#[allow(unused_imports)]
use ncurses::*;
use parser::Parser;
use stage::Stage;

use crate::lexer::TokenKind;

pub mod game;
pub mod lexer;
pub mod parser;
pub mod stage;

fn _test_parser() {
    let src = r#"open the door"#;

    let mut parser = Parser::new(src);

    loop {
        let expr = match parser.next().unwrap() {
            Some(t) => t,
            None => break,
        };
        dbg!(expr);
    }
}

fn main() {
    initscr();

    cbreak();
    noecho();
    initscr();
    scrollok(stdscr(), true);
    keypad(stdscr(), true);

    let mut s = String::new();
    let mut x = 0;
    let mut y = 0;

    let mut max_x = 0;
    let mut max_y = 0;

    let mut game = Game::new();

    game.transition(Stage::First);

    while game.is_running() {
        getyx(stdscr(), &mut y, &mut x);
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        loop {

            getyx(stdscr(), &mut y, &mut x);
            let c = getch() as u8 as char;
            if c == '\n' {
                y += 1;
                wmove(stdscr(), y, 0);
                break;
            }
            s.push(c);
            x += 1;

            /*
            let c = match get_wch() {
                Some(c) => c,
                None => break,
            };

            match c {
                WchResult::KeyCode(constants::KEY_LEFT) if x > 0 => {
                    x -= 1;
                }
                WchResult::KeyCode(constants::KEY_RIGHT) if (x as usize) < s.len() => {
                    x += 1;
                }
                WchResult::KeyCode(constants::KEY_BACKSPACE) if x > 0 => {
                    x -= 1;
                    s.remove(x as usize);
                }
                WchResult::Char(c) if char::from_u32(c).unwrap() == '\n' => {
                    y += 1;
                    wmove(stdscr(), y, 0);
                    break;
                }
                WchResult::Char(c) => {
                    s.insert(x as usize, char::from_u32(c).unwrap());
                    x += 1;
                }
                _ => {
                }
            }
            */
            wmove(stdscr(), y, 0);
            clrtoeol();
            addstr(&s);
            wmove(stdscr(), y, x);
            refresh();
        }

        if y >= max_y {
            wmove(stdscr(), y + 1, 0);
            scrl(10);
        }

        game.eval(&s);
        refresh();
        s.clear();
    }
    endwin();
}
