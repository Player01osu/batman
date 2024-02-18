use crate::game::{msg, oops};
use crate::{game::Game, parser::GameExpr};
use crate::lexer::VerbKind;
use crate::lexer::NounKind;
use crate::lexer::AdjKind;
use crate::lexer::AdverbKind;
use super::Stage;

impl Game {
    pub fn eval_bus_arrive(&mut self, game: GameExpr) -> Stage {
        let (verb, noun, adverb, adj) = match game {
            GameExpr::Svn { verb, noun, adverb, adj } => (verb, noun, adverb, adj),
            _ => unreachable!(),
        };

        match (verb, noun) {
            (VerbKind::Do, NounKind::Nothing) | (VerbKind::Wait, NounKind::Bus) => {
                msg("You stand in front of the bus like an idiot\nand it takes off without you");
                self.adjust_time(-4);
                Stage::TransitOnFoot
            }
            (VerbKind::Enter, NounKind::Bus) => {
                Stage::BusFire
            }
            _ => {
                oops();
                self.stage
            }
        }
    }
}

