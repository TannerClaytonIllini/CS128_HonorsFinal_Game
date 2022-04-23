pub mod cards;
pub mod Blackjack;
pub mod BlackjackAI;
pub mod Player;
pub mod UI;
use crate::cards::*;
use crate::Blackjack::*;
use crate::BlackjackAI::*;
use crate::Player::*;
use crate::AI::*;
use crate::menu::*;

fn main() {
    menu::main();
    
    /*
    let test: Card = BuildCard("new");
    println!("{}",test.name_);
    let Hand1: Vec<Card> = BuildDeck();
    let mut count = 0;
    for card in Hand1 {
        println!("{}: {}", card.name_, card.value_);
        count += 1;
    }
    println!("Total Cards: {}", count);
    */
}

