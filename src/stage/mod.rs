use crate::game::{msg, Game, ParseMode};
use ncurses::*;

pub mod first;
pub mod library;

#[derive(Debug, Clone, Copy)]
pub enum Stage {
    First,
    PlayConfirm,
    Library,
    OutsideLibrary,
    Quit,
}

fn print_confirm_hint() {
    addstr("Try: 'yes' or 'no'\n");
}

pub fn possible_nouns(nouns: &[&str]) {
    addstr("Here are some other things around you...\n");
    for noun in nouns {
        addstr(noun);
        addstr("\n");
    }
}

#[derive(Debug, Clone)]
pub struct State {
    name: String,
    time_left: i32,
    health: u32,
    armor: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: "Jeff".to_string(),
            time_left: 60,
            health: 10,
            armor: 0,
        }
    }
}

impl Game {
    pub fn print_time_left(&mut self) {
        msg(&format!("You have {} minutes left...\n", self.state.time_left));
    }

    pub fn adjust_time(&mut self, delta: i32) {
        self.state.time_left += delta;
        self.print_time_left();
    }

    pub fn transition(&mut self, stage: Stage) {
        self.parse_mode = self.transition_aux(stage);
        self.stage = stage;
    }

    pub fn transition_aux(&mut self, stage: Stage) -> ParseMode {
        clear();
        match stage {
            Stage::First => {
                addstr("Welcome! What is your name\n");
                self.name = "Jeff".to_string();
                ParseMode::Raw
            }
            Stage::PlayConfirm => {
                addstr("Would you like to play the game?\n");
                ParseMode::Confirm
            }
            Stage::Library => {
                addstr("The setting is Houghton, mid January...\n");
                addstr("Your class begins in an hour...\n");
                addstr("You are in the Library Resturant and would like to go back to campus...\n");
                addstr("What should you do?\n");
                ParseMode::Grammar
            }
            Stage::OutsideLibrary => {
                addstr("You swing open the door and are hit with a big gust of wind...\n");
                // TODO if equip coat, say how it's fine
                addstr("What now?\n");
                ParseMode::Grammar
            }
            Stage::Quit => {
                addstr("Bye!\n");
                self.is_running = false;
                ParseMode::Raw
            }
        }
    }

    pub fn print_hint(&self) {
        match self.stage {
            Stage::First => {
                addstr("Just type your name...\n");
            }
            Stage::PlayConfirm => {
                print_confirm_hint();
            }
            Stage::Library => {
                addstr("Try: 'equip coat' or 'leave the building'\n");
                possible_nouns(&["coat", "building", "sword", "bathroom"]);
            }
            Stage::OutsideLibrary => {
                addstr("Around you is a sea of white\n");
                addstr("But knowing your bus is coming soon, you can either\n");
                addstr("wait, or continue on foot...\n");
                possible_nouns(&["snow", "library", "campus"]);
            }
            Stage::Quit => unreachable!(),
        }
    }
}
