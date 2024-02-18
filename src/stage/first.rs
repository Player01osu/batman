use crate::{game::Game, parser::GameExpr};
use crate::game::Result;
use ncurses::*;

use super::{Stage, State};

impl Game {
    pub fn eval_first(&mut self, game: GameExpr) -> Stage {
        self.state = State::default();
        match game {
            GameExpr::Raw(name) => {
                self.state.name = name.to_string();
            },
            _ => {
                self.state.name = "Jeff".to_string();
            },
        }

        addstr(&format!("Greetings, {}\n", self.state.name));

        Stage::PlayConfirm
    }

    pub fn eval_playconfirm(&mut self, game: GameExpr) -> Stage {
        match game {
            GameExpr::Confirm(true) => {
                Stage::Library
            }
            GameExpr::Confirm(false) => {
                Stage::Quit
            }
            _ => unreachable!()
        }
    }
}
