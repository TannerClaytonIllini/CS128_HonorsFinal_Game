use super::cards::*;
use super::Player::*;
use std::io::{stdin,stdout,Write};
use rand::Rng;
use std::thread;
use std::thread::JoinHandle;

/* Holds game data */
pub struct Game {
    fulldeck: Vec<Card>,
    players: Vec<Player>,
}

pub fn main() {
    println!("Welcome to Blackjack!");
    println!("This game assumes you are already are familiar with Blackjack and currently doesn't explain how play.\nIf doesn't work for you, indly Combust.\nHave a Great Game!");
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
    //Game Begin
    let mut gamestate: Game = Setup(playercount as u8, deckcount as u8);
    let mut gameplaycondition: bool = true;
    println!("BlackJack main");
    while gameplaycondition { //if game is active :: a single hand
        InitialDeal(&mut gamestate);
        let mut round: bool = true;
        while round {
            //let mut handles: Vec<JoinHandle<()>> = vec![];
            for player in &gamestate.players {
                /* used later to distribut display to multipule human players
                let mut playersc: Vec<Player> = &gamestate.clonePlayers();
                let mut playerclone: Player = player.clone();
                let handle = thread::spawn(move || {
                    DisplayGameState(playersc, &playerclone);
                });
                handles.push(handle);
                */
                DisplayGameState(&gamestate.players, &player);
                println!("NEXT PLAYER");
            }
            round = false;
        }
        // test / escape latch
        let s: String = GetInput("If done type 'escape': ");
        if s == "escape".to_string() {
            gameplaycondition = false;
        }
        println!("You typed: {}",s);
    }
    println!("Game Ended");
}

/* Begins the game. Sets up Player objects and Deck for the game */
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
        tplayers.push(super::Player::BuildPlayerID(count as i32));
        count += 1;
    }
    Game {
        fulldeck: tDealDeck,
        players: tplayers,
    }
}

/* First Deal of the game sets up a hand for each player */
pub fn InitialDeal(game: &mut Game) {
    let mut count = 0;
    while count < 2 { //deals 2 cards to each player
        for player in &mut game.players {
            let mut rng = rand::thread_rng();
            player.hand.push(game.fulldeck.remove(rng.gen_range(0..(game.fulldeck.len() as i32) as usize)));
        }
        count += 1;
    }
}



pub fn DisplayGameState(players: &Vec<Player>, curr: &Player) {
    for player in players {
        if player == curr {
            print!("Your hand: ");
            for card in &player.hand {
                print!("{}, ", card.name_);
            }
            print!("\n");
            continue;
        }
        print!("Player {} hand: ", player.id);
        for card in &player.hand {
            if card == &player.hand[0] {
                print!("'hidden' ");
                continue;
            }
            print!("{} ", card.name_);
        }
        print!("\n");
    }
}

impl Game {
    pub fn clonePlayers(&self) -> Vec<Player> {
        let mut outvec: Vec<Player> = vec![];
        for player in &self.players {
            outvec.push(player.clone())
        }
        return outvec;
    }
}

/* Whenever needs player input. returns string of players input/choice. Param - Output prompt to the player */
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


/* Returns number of players */
pub fn GetPlayers(game: Game) -> u8 {
    return game.players.len() as u8;
}

pub fn DealHands(gamestate: Game, players: Vec<Player>) {

}