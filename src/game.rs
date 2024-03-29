use std::collections::HashSet;

use ncurses::*;

use crate::{lexer::{NounKind, VerbKind}, parser::{Expr, GameExpr, ParseErr, Parser, ProgramExpr}, stage::{Stage, State}};

#[derive(Debug)]
pub struct Game {
    pub is_running: bool,
    pub parse_mode: ParseMode,
    pub name: String,
    pub stage: Stage,
    pub state: State,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Equipment {
    Coat,
    Sword
}

#[derive(Clone, Debug)]
pub enum GameErr {
    Parse(ParseErr),
}

pub type Result<T> = std::result::Result<T, GameErr>;

impl From<ParseErr> for GameErr {
    fn from(value: ParseErr) -> Self {
        Self::Parse(value)
    }
}

const HELP: &str = include_str!("../help.txt");

#[derive(Debug, Clone, Copy)]
pub enum ParseMode {
    Grammar,
    Raw,
    Confirm,
}

pub fn msg(s: &str) {
    clear();
    addstr(&format!("{s}\nPress any key to continue...\n"));
    getch();
}

pub fn oops() {
    msg("Can't use that here; try 'help' or 'hint'\n");
}

impl Game {
    pub fn new() -> Self {
        Self {
            is_running: true,
            parse_mode: ParseMode::Grammar,
            name: "Jeff".to_string(),
            stage: Stage::First,
            state: Default::default(),
        }
    }

    pub fn print_help(&mut self) {
        clear();
        addstr(&format!("{HELP}\n\nPress any key to continue..."));
        getch();
        refresh();
        self.transition(self.stage);
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn equip_armor(&mut self, armor: Equipment, value: u32, s: &str) {
        if self.state.equipment.insert(armor) {
            msg(s);
            self.state.armor += value;
        } else {
            msg("You already have this equipt");
        }
    }

    fn eval_program_exit(&mut self, program: ProgramExpr) {
        match program.noun() {
            NounKind::Game => self.is_running = false,
            _ => unimplemented!(),
        }
    }

    fn eval_program(&mut self, program: ProgramExpr) {
        match program.verb() {
            VerbKind::Exit => self.eval_program_exit(program),
            _ => unimplemented!()
        }
    }

    fn eval_game_over(&mut self, game: GameExpr) -> Stage {
        let confirm = match game {
            GameExpr::Confirm(b) => b,
            _ => unreachable!(),
        };

        if confirm {
            Stage::First
        } else {
            Stage::Quit
        }
    }


    pub fn eval_finish(&mut self, game: GameExpr) -> Stage {
        let confirm = match game {
            GameExpr::Confirm(b) => b,
            _ => unreachable!(),
        };

        if confirm {
            Stage::First
        } else {
            Stage::Quit
        }
    }

    fn eval_game(&mut self, game: GameExpr) {
        let next_stage = match self.stage {
            Stage::First => {
                self.eval_first(game)
            }
            Stage::PlayConfirm => {
                self.eval_playconfirm(game)
            }
            Stage::Library => {
                self.eval_library(game)
            }
            Stage::OutsideLibrary => {
                self.eval_outside_library(game)
            }
            Stage::TransitOnFoot => {
                Stage::CampusDragon
            }
            Stage::BusArrive => {
                self.eval_bus_arrive(game)
            }
            Stage::GameOver => {
                self.eval_game_over(game)
            }
            Stage::BusFire => {
                self.eval_bus_fire(game)
            }
            Stage::CampusDragon => {
                self.eval_campus_dragon(game)
            }
            Stage::StrikeDragon => {
                self.eval_strike_dragon(game)
            }
            Stage::Quit => {
                Stage::Quit
            }
            Stage::Finish => {
                self.eval_finish(game)
            },
        };
        self.transition(next_stage);
    }

    pub fn eval(&mut self, s: &str) {
        match s.trim().to_ascii_lowercase().as_str() {
            "help" => {
                self.print_help();
                return;
            }
            "hint" => {
                self.print_hint();
                return;
            }
            _ => ()
        }

        let expr = {
            match self.parse_mode {
                ParseMode::Raw => {
                    Expr::Game(GameExpr::Raw(s.to_string()))
                }
                ParseMode::Grammar => {
                    let mut parser = Parser::new(s);
                    match parser.next() {
                        Ok(Some(v)) => v,
                        Ok(None) => return,
                        Err(ParseErr::Unexpected((_, _))) => {
                            addstr("Unknown command, try 'help' or 'hint'\n");
                            return;
                        }
                        Err(ParseErr::Unimplemented) => {
                            return;
                        }
                        Err(e) => {
                            addstr(e.to_string().as_str());
                            addstr("\n");
                            return;
                        }
                    }
                }
                ParseMode::Confirm => {
                    match s.trim().to_ascii_lowercase().as_str() {
                        "yes" | "true" | "ok" => Expr::Game(GameExpr::Confirm(true)),
                        "no" | "false" | "nope" => Expr::Game(GameExpr::Confirm(false)),
                        _ => {
                            addstr("Invalid option: try yes or no\n");
                            return
                        }
                    }
                }
            }
        };

        match expr {
            Expr::Game(game) => {
                self.eval_game(game);
            }
            Expr::Program(program) => {
                self.eval_program(program);
            }
            Expr::Help => self.print_help(),
            Expr::Hint => todo!(),
            _ => unimplemented!(),
        }
    }
}
