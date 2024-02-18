use ncurses::*;

use crate::{lexer::{NounKind, VerbKind}, parser::{Expr, GameExpr, ParseErr, Parser, ProgramExpr}};

#[derive(Debug)]
pub struct Game {
    is_running: bool,
    no_parse: bool,
    name: String,
    stage: Stage,
}

#[derive(Debug, Clone, Copy)]
pub enum Stage {
    First
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

fn print_help() {
    addstr(HELP);
}

impl Game {
    pub fn new() -> Self {
        Self {
            is_running: true,
            no_parse: false,
            name: "Jeff".to_string(),
            stage: Stage::First,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    fn eval_program_exit(&mut self, program: ProgramExpr) -> Result<()> {
        match program.noun() {
            NounKind::Game => self.is_running = false,
            _ => unimplemented!(),
        }
        Ok(())
    }

    fn eval_program(&mut self, program: ProgramExpr) -> Result<()> {
        match program.verb() {
            VerbKind::Exit => self.eval_program_exit(program),
            _ => unimplemented!()
        }
    }

    fn eval_raw(&mut self, s: String) -> Result<()> {
        Ok(())
    }

    fn eval_game(&mut self, game: GameExpr) -> Result<()> {
        match self.stage {
            Stage::First => {
                self.eval_first(game)
            }
        }
    }

    pub fn eval(&mut self, s: &str) -> Result<()> {
        let expr = {
            if self.no_parse {
                self.no_parse = false;
                Expr::Raw(s.to_string())
            } else {
                let mut parser = Parser::new(s);
                match parser.next()? {
                    Some(v) => v,
                    None => return Ok(()),
                }
            }
        };

        match expr {
            Expr::Raw(raw) => {
                self.eval_raw(raw)?;
            }
            Expr::Game(game) => {
                self.eval_game(game)?;
            }
            Expr::Program(program) => {
                self.eval_program(program)?;
            }
            Expr::Help => print_help(),
            Expr::Hint => todo!(),
            _ => unimplemented!(),
        }
        Ok(())
    }
}
