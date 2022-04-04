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
    println!("This game assumes you are already are familiar with Blackjack and currently doesn't explain how play.\nIf that doesn't work for you, kindly Combust.\nHave a Great Game!");
    println!("We ask that you only input inputs when asked EXACTLY in the form requested: exactly the phrase within '_' ");
    println!("Example: 'hit' or 'pass' .\n Your input should be:\nhit\nExactly what is within the hyponses but without the hyphones themselves");
    println!("=====================================================");
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
    let mut gameplaycondition: bool = true;
    println!("BlackJack main");
    while gameplaycondition { //if game is active :: a single hand
        let mut round: bool = true;
        while round {
            let mut gamestate: Game = Setup(playercount as u8, deckcount as u8);
            InitialDeal(&mut gamestate); //add check for how many cards left in the deck
            println!("\n=============== NEW ROUND ================\n");
            //let mut handles: Vec<JoinHandle<()>> = vec![];
            for player in &gamestate.players {
                /* // used later to distribute display to multipule human players
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
            //game active after initial deal
            for player in &mut gamestate.players { //This can be made into a funciton
                println!("It's your turn Player {}!",player.id);
                let mut curractive = true; //current player is still active / has not passed yet
                while curractive {
                    let total = player.TotalHand();
                    if total <= 21 as u8 {
                        print!("Hand: ");
                            player.DisplayHand();
                        let mut choice = GetInput("'hit'(get another card) or 'pass'(end your hand)\n");
                        if choice == "hit".to_string() {
                            let mut rng = rand::thread_rng();
                            let card = gamestate.fulldeck.remove(rng.gen_range(0..(gamestate.fulldeck.len() as i32) as usize));
                            println!("{}", card.name_);
                            player.hand.push(card);
                            if player.TotalHand() > 21 as u8 {
                                println!("You went over 21 and are out of the game for the rest of the round!");
                                curractive = false;
                            }
                        } else if choice == "pass".to_string() {
                            curractive = false;
                            println!("You passed next players turn")
                        }
                    }
                }
                //Get winner
            }
            round = false;
        }
        // test / escape latch
        let s: String = GetInput("If done type 'end', enter anything else to play another round with the same settings\n");
        if s == "end".to_string() {
            gameplaycondition = false;
        }
    }
    println!("================== Game Ended ===================");
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
            player.DisplayHand();
            continue;
        }
        print!("Player {} hand: ", player.id);
        for card in &player.hand {
            if card == &player.hand[0] {
                print!("'hidden' ");
                continue;
            }
            print!("{}, ", card.name_);
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