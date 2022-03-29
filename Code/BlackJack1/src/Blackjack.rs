use super::cards::*;
use super::Player::*;
use std::io::{stdin,stdout,Write};

pub struct Game {
    DealDeck: Vec<Card>,
    players: Vec<Player>,
}

pub fn main() {
    println!("Welcome to Blackjack!");
    //PreGame Setup
    let mut setlatch: bool = true;
    let mut playercount = 1;
    let mut deckcount = 1;
    while setlatch {
        playercount = GetInput("You can play with up to 1-8 players, including yourself.\nHow many players: ").parse::<i32>().unwrap(); 
        if playercount > 0 && playercount < 9 {
            setlatch = false;
        }
    }
    setlatch = true;
    while setlatch {
        deckcount = GetInput("You can play with up to 1-7 decks.\nHow many decks: ").parse::<i32>().unwrap(); 
        if deckcount > 0 && deckcount < 8 {
            setlatch = false;
        }
    }
    let gamestate: Game = Setup(playercount as u8, deckcount as u8);
    //Game Begin
    let mut gameplaycondition: bool = true;
    println!("BlackJack main");
    while gameplaycondition { //if game is active
        let s: String = GetInput("A number is: ");
        if s == "escape".to_string() {
            gameplaycondition = false;
        }
        println!("You typed: {}",s);
    }
    println!("Game Ended");
}

pub fn Setup(players: u8, decks: u8) -> Game {
    if (players == 0) || (decks == 0) {
        println!("players and decks must be created with a minimum of one");
    }
    let mut tDealDeck: Vec<Card> = vec![];
    let mut count = 1;
    while count <= decks {
        count += 1;
        let tempDeck = super::cards::BuildDeck();
        for card in tempDeck {
            tDealDeck.push(card);
        }
    }
    count = 1;
    let mut tplayers = vec![];
    while count <= players {
        tplayers.push(super::Player::BuildPlayer());
        count += 1;
    }
    Game {
        DealDeck: tDealDeck,
        players: tplayers,
    }
}

pub fn GetInput(outtext: &str) -> String{
    let mut inputstring = String::new();
    print!("{}", outtext);
    let _=stdout().flush();
    stdin().read_line(&mut inputstring).expect("Did not enter a correct string");
    if let Some('\n')=inputstring.chars().next_back() {
        inputstring.pop();
    }
    if let Some('\r')=inputstring.chars().next_back() {
        inputstring.pop();
    }
    return inputstring;
}

pub fn GetPlayers(game: Game) -> u8 {
    return game.players.len() as u8;
}

pub fn DealHands(gamestate: Game, players: Vec<Player>) {

}