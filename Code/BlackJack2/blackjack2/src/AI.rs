//AI for singleplayer or fillers
//used later after game if almost done. Same production stage as player, probably before networking

use super::cards::*;
use super::Player::*;
use super::BlackjackAI::Game;

pub struct AI {
    pub hand: Vec<Card>,
    pub id: i32,
    pub difficulty: u8,
}

impl AI {
    pub fn clone(&self) -> AI {
        let mut chand: Vec<Card> = vec![];
        for card in &self.hand {
            chand.push(card.clone());
        }
        AI {
            hand: chand,
            id: self.id.clone(),
            difficulty: self.difficulty.clone(),
        }
    }

    pub fn DisplayHand(&self) {
        print!("AI {} hand: ", &self.id);
        for card in &self.hand {
            print!("{}, ", card.name_);
        }
        print!("| Total Value: {}", &self.TotalHand());
        print!("\n");
    }
    pub fn TotalHand(&self) -> u8 {
        let mut total: u8 = 0;
        for card in &self.hand {
            total += card.value_;
        }
        return total;
    }
    pub fn AIturn(&self, deck: &Vec<Card>) -> String {
        let curr_limit = 21 - self.TotalHand();
        let mut possible_cards = 0;
        for card in deck {
            if card.value_ <= curr_limit {
                possible_cards += 1;
            }
        }
        let probablility = possible_cards / deck.len();
        match self.difficulty {
            2 => {
                if probablility >= 0.6 as usize {
                    return "hit".to_string();
                } else {
                    return "pass".to_string();
                }
            }
            3 => {
                if probablility >= 0.4 as usize {
                    return "hit".to_string();
                } else {
                    return "pass".to_string();
                }
            }
            _ => {
                if probablility >= 0.2 as usize {
                    return "hit".to_string();
                } else {
                    return "pass".to_string();
                }
            }
        }
    }
}

/* creates new AI */
pub fn BuildAI() -> AI{
    AI {
        hand: vec![],
        id: 0,
        difficulty: 1,
    }
    
}
/* creates new AI with specified 'count'/integer id */
pub fn BuildAIinfo(id: i32, difficulty: u8) -> AI{
    AI {
        hand: vec![],
        id: id,
        difficulty: difficulty,
    }
}