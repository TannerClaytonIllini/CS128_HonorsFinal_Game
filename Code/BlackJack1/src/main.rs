pub mod cards;
pub mod Blackjack;
pub mod BlackjackAI;
pub mod Player;
<<<<<<< HEAD
pub mod AI;
pub mod menu;
=======
pub mod UI;
>>>>>>> 80ed4ea77da332e1f8cbaf60491879b185b436ea
use crate::cards::*;
use crate::Blackjack::*;
use crate::BlackjackAI::*;
use crate::Player::*;
use crate::AI::*;
use crate::menu::*;

fn main() {
<<<<<<< HEAD
    menu::main();
=======
    UI::main();
    
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
>>>>>>> 80ed4ea77da332e1f8cbaf60491879b185b436ea
}

