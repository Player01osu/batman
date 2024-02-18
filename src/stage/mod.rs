use std::collections::HashSet;

use crate::game::{msg, Equipment, Game, ParseMode};
use ncurses::*;

pub mod first;
pub mod library;
pub mod outside_library;
pub mod campus_dragon;
pub mod bus_fire;
pub mod bus_arrive;

#[derive(Debug, Clone, Copy)]
pub enum Stage {
    First,
    PlayConfirm,
    Library,
    OutsideLibrary,
    TransitOnFoot,
    BusArrive,
    GameOver,
    CampusDragon,
    BusFire,
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
    pub name: String,
    pub time_left: i32,
    pub health: u32,
    pub armor: u32,
    pub equipment: HashSet<Equipment>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: "Jeff".to_string(),
            time_left: 60,
            health: 10,
            armor: 0,
            equipment: HashSet::new(),
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

    pub fn transition(&mut self, mut stage: Stage) {
        self.parse_mode = self.transition_aux(&mut stage);
        self.stage = stage;
    }

    pub fn transition_aux(&mut self, stage: &mut Stage) -> ParseMode {
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
            Stage::BusFire => {
                addstr("You board the bus headed to campus...\n");
                addstr("Everything is going fine until...\n");
                addstr("The bus becomes engulfed in flames!\n");
                ParseMode::Grammar
            }
            Stage::OutsideLibrary => {
                addstr("You swing open the door and are hit with a big gust of wind...\n");
                // TODO if equip coat, say how it's fine
                addstr("What now?\n");
                ParseMode::Grammar
            }
            Stage::BusArrive => {
                addstr("After some time, the bus arrives...\n");
                addstr("A new decision bestows you...\n");
                ParseMode::Grammar
            }
            Stage::TransitOnFoot => {
                addstr("You're too good for a bus...\n");
                if self.state.equipment.contains(&Equipment::Coat) {
                    addstr("You decide to walk instead...\n");
                    *stage = Stage::CampusDragon;
                    self.transition_aux(stage)
                } else {
                    addstr("As you tread through the snow, you feel\n");
                    addstr("your legs weaken as you become enveloped in cold...\n");
                    msg("You have frozen to death");
                    *stage = Stage::GameOver;
                    self.transition_aux(stage)
                }
            }
            Stage::CampusDragon => {
                addstr("You made it to the campus...\n");
                addstr("However, a dragon blocks your way...\n");
                ParseMode::Grammar
            }
            Stage::GameOver => {
                addstr("Unfortunately you have game overed...\n");
                addstr("If you would like to try again, type 'yes'\n");
                ParseMode::Confirm
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
            Stage::TransitOnFoot => {
            }
            Stage::BusArrive => {
                possible_nouns(&["bus"]);
            }
            Stage::GameOver => {
            }
            Stage::BusFire => {
                possible_nouns(&["extinguisher"]);
            }
            Stage::CampusDragon => {
                let mut nouns = vec!["fist", "pen"];
                if self.state.equipment.contains(&Equipment::Sword) {
                    nouns.push("sword");
                }
                possible_nouns(nouns.as_slice());
            }
        }
    }
}
