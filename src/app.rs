use std::fmt;
use rand::prelude::*;

//enum to hold game screens
pub enum GameScreen {
    Choice,
    Playing,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameChoice {
    Rock,
    Paper,
    Scissor,
}

pub struct Score {
    wins: u8,
    loss: u8,
    tie: u8,
}

pub struct App {
    pub game_score: Score,
    pub current_game_screen: GameScreen, //state of screen
    pub current_game_choice: Option<GameChoice>, //state of the current game choice, set to option so it can be empty before/after game
    pub selected_tab: usize,
}

//helper functions, should only affect state
impl App {
    //creates new instance of app
    pub fn new() -> App {
        App {game_score: Score { wins: 0, loss: 0, tie: 0}, current_game_screen: GameScreen::Choice, current_game_choice: None, selected_tab: 0}
    }

    pub fn print_score(&self){
        println!("win: {}, loss: {}, tie: {}", self.game_score.wins, self.game_score.loss, self.game_score.tie);
    }

    pub fn play_game(&mut self) {
        let player_choice = match self.current_game_choice {
            Some(item) => {item}
            None => todo!()
        };
        let mut rng = rand::rng();
        let rand_choice_num = rng.random_range(0..=2);
        let rand_choice: GameChoice;
        match rand_choice_num {
            0 => {rand_choice = GameChoice::Rock}
            1 => {rand_choice = GameChoice::Paper}
            2 => {rand_choice = GameChoice::Scissor}
            i32::MIN..=-1_i32 => {rand_choice = GameChoice::Rock}
            3_i32..=i32::MAX => {rand_choice = GameChoice::Scissor}
        }
        //println!("{:?}",rand_choice);

        if rand_choice == player_choice {
            self.game_score.tie += 1; 
        } else if player_choice == GameChoice::Rock && rand_choice == GameChoice::Scissor {
            self.game_score.wins += 1; //rock beats scissors
        } else if player_choice == GameChoice::Paper && rand_choice == GameChoice::Rock {
            self.game_score.wins += 1; //paper beats rock 
        } else if player_choice == GameChoice::Scissor && rand_choice == GameChoice::Paper {
            self.game_score.wins += 1;
        } else {
            self.game_score.loss += 1;
        }
        //scissors beats paper

    } 
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wins: {}, Losses: {}, Ties: {}", self.wins, self.loss, self.tie)
    }
}