

use super::Blackjack::*;
use super::BlackjackAI::*;



pub fn main() {
    println!("Welcome to Blackjack!");
    println!("This game assumes you are already are familiar with Blackjack and currently doesn't explain how play.\nIf that doesn't work for you, kindly Combust.\nHave a Great Game!");
    println!("We ask that you only input inputs when asked EXACTLY in the form requested: exactly the phrase within '_' ");
    println!("Example: 'hit' or 'pass' .\n Your input should be:\nhit\nExactly what is within the hyponses but without the hyphones themselves");
    println!("=====================================================");
    let mut option = 0;
    let mut setlatch = true;
    while setlatch {
        option = super::Blackjack::GetInput("You can play against AIs'(input '1') or players locally on this terminal(multiplayer)(input '2')").parse::<i32>().unwrap();
        //can add third(3) network multiplayer
        if option >= 0 && option < 3 {
            setlatch = false;
        }
    }
    match option {
        1 => {super::BlackjackAI::main();},
        _ => {super::Blackjack::main();},
    }
}