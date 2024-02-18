use crate::game::{msg, oops};
use crate::{game::Game, parser::GameExpr};
use crate::lexer::VerbKind;
use crate::lexer::NounKind;
use crate::lexer::AdjKind;
use crate::lexer::AdverbKind;
use super::Stage;

impl Game {
    pub fn eval_bus_fire(&mut self, game: GameExpr) -> Stage {
        let (verb, noun, adverb, adj) = match game {
            GameExpr::Svn { verb, noun, adverb, adj } => (verb, noun, adverb, adj),
            _ => unreachable!(),
        };

        match (verb, noun) {
            (VerbKind::Run, NounKind::Away) => {
                msg("You try to exit the front of the bus, but the entrance is blocked!");
                self.stage
            }
            _ => {
                oops();
                self.stage
            }
        }
    }
}

