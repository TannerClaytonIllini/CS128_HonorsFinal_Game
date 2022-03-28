pub mod cards;
pub mod Blackjack;
pub mod Player;
use crate::cards::*;
use crate::Blackjack::*;
use crate::Player::*;

fn main() {
    Blackjack::main();
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
