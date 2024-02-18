use crate::game::{msg, oops, Equipment};
use crate::{game::Game, parser::GameExpr};
use crate::lexer::VerbKind;
use crate::lexer::NounKind;
use crate::lexer::AdjKind;
use crate::lexer::AdverbKind;
use super::Stage;

impl Game {
    pub fn do_damage(&mut self, n: u32) {
        let damage = n as f32 * ((100 - self.state.armor) as f32 / 100.0);
        self.state.health -= damage as i32;
    }


    pub fn eval_strike_dragon(&mut self, game: GameExpr) -> Stage {
        let (verb, noun, adverb, adj) = match game {
            GameExpr::Svn { verb, noun, adverb, adj } => (verb, noun, adverb, adj),
            _ => unreachable!(),
        };

        match (verb, noun) {
            (VerbKind::Strike, NounKind::Sword) => {
                if self.state.equipment.contains(&Equipment::Sword) {
                    msg("You strike the dragon with a great sword...\n");
                    msg("It kills the dragon...\n");
                    Stage::Finish
                } else {
                    msg("You do not have a sword!\n");
                    msg("The dragon attacks you!\n");
                    self.do_damage(4);
                    self.stage
                }
            }
            _ => {
                oops();
                self.stage
            }
        }
    }
}

