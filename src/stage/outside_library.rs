use crate::game::{msg, oops};
use crate::{game::Game, parser::GameExpr};
use crate::lexer::VerbKind;
use crate::lexer::NounKind;
use crate::lexer::AdjKind;
use crate::lexer::AdverbKind;
use super::Stage;

fn go_on_foot(verb: VerbKind, noun: NounKind) -> bool {
    matches!(verb, VerbKind::Go | VerbKind::Walk | VerbKind::Head) &&
        matches!(noun, NounKind::Campus)
}

impl Game {
    pub fn eval_outside_library(&mut self, game: GameExpr) -> Stage {
        let (verb, noun, adverb, adj) = match game {
            GameExpr::Svn { verb, noun, adverb, adj } => (verb, noun, adverb, adj),
            _ => unreachable!(),
        };

        match (verb, noun) {
            (VerbKind::Do, NounKind::Nothing) | (VerbKind::Wait, NounKind::Bus) => {
                msg("You decide to wait for the bus...\n");
                self.adjust_time(-10);
                Stage::BusArrive
            }
            _ if go_on_foot(verb, noun) => {
                Stage::TransitOnFoot
            }
            _ => {
                oops();
                self.stage
            }
        }
    }
}

