use crate::game::msg;
use crate::game::oops;
use crate::game::Game;
use crate::parser::GameExpr;
use crate::lexer::VerbKind;
use crate::lexer::NounKind;
use crate::lexer::AdjKind;
use crate::lexer::AdverbKind;

use super::Stage;

impl Game {
    pub fn eval_library(&mut self, game: GameExpr) -> Stage {
        let (verb, noun, adverb, adj) = match game {
            GameExpr::Svn { verb, noun, adverb, adj } => (verb, noun, adverb, adj),
            _ => unreachable!(),
        };

        match (verb, noun) {
            (VerbKind::Leave, NounKind::Building) => {
                match adverb {
                    Some(AdverbKind::Slow) => {
                        msg("You decide to leave the building at a slow pace...\n");
                        self.adjust_time(-10);
                    }
                    Some(AdverbKind::Quick) => {
                        msg("You decide to leave the building at a quick pace...\n");
                        self.adjust_time(-1);
                    }
                    _ => {
                        msg("You decide to leave the building at a moderate pace...\n");
                        self.adjust_time(-5);
                    }
                }
                Stage::OutsideLibrary
            },
            (VerbKind::Do, NounKind::Nothing) => {
                msg("You decide to kill some time...\n");
                self.adjust_time(-5);
                Stage::Library
            }
            (_, NounKind::Bathroom) => {
                match verb {
                    VerbKind::Enter => {
                        msg("You check yourself out in the mirror...\n");
                        msg("Man, you look gorgeous...\n");
                        self.adjust_time(-5);
                    }
                }
                Stage::Library
            }
            _ => {
                oops();
                self.stage
            }
        }
    }
}
