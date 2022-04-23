use super::cards::*;
use super::Player::*;
use super::AI::*;
use std::io::{stdin,stdout,Write};
use rand::Rng;
use std::thread;
use std::thread::JoinHandle;

pub struct Game {
    fulldeck: Vec<Card>,
    player: Player,
    aiplayers: Vec<AI>,
}

pub fn main() {
    //PreGame Setup
    let mut setlatch: bool = true;
    let mut aicount = 1;
    let mut deckcount = 1;
    while setlatch {
        aicount = GetInput("You can play with up to 1-7 AIs.\nHow many AIs: ").parse::<i32>().unwrap(); 
        if aicount > 0 && aicount < 8 {
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
    println!("BlackJackAI main");
    while gameplaycondition { //if game is active :: a single hand
        let mut round: bool = true;
        while round {
            let mut gamestate: Game = Setup(aicount as u8, deckcount as u8);
            InitialDeal(&mut gamestate); //add check for how many cards left in the deck
            println!("\n=============== NEW ROUND ================\n");
            //ADD a deck condidtion and adjustment for at least 5 cards per hand minimum.
            //let mut handles: Vec<JoinHandle<()>> = vec![];
            DisplayGameState(&gamestate.aiplayers, &gamestate.player);
            //game active after initial deal
            for ai in &mut gamestate.aiplayers { //This can be made into a funciton
                println!("It's AI {} turn!",ai.id);
                let mut curractive = true; //current player/ai is still active / has not passed yet
                while curractive {
                   // let clonedeck: &Vec<Card> = &gamestate.cloneDeck();
                    if ai.TotalHand() <= 21 as u8 {
                            ai.DisplayHand();
                        let choice: String = "pass".to_string(); //ai.AIturn(clonedeck); // need to fix reference issue
                        if choice == "hit".to_string() {
                            let mut rng = rand::thread_rng();
                            let card = gamestate.fulldeck.remove(rng.gen_range(0..(gamestate.fulldeck.len() as i32) as usize));
                            println!("{}", card.name_);
                            ai.hand.push(card);
                            if ai.TotalHand() > 21 as u8 {
                                println!("AI {} went over 21 and is out of the game for the rest of the round!", ai.id);
                                curractive = false;
                            }
                        } else if choice == "pass".to_string() {
                            curractive = false;
                            println!("You passed next players turn")
                        }
                    }
                }
            }
            GetWinner(&gamestate.aiplayers, &gamestate.player, &mut round);
            //round = false;
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
pub fn Setup(ais: u8, decks: u8) -> Game {
    if (ais == 0) || (decks == 0) {
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
    let mut tais: Vec<AI> = vec![];
    while count <= ais {
        let mut setlatch = true;
        let mut difficulty: u8 = 0;
        while setlatch {
            difficulty = GetInput("Please select AI{}'s difficulty: '1'-EASY , '2'-MEDIUM , '3'-HARD\n", count).parse::<u8>().unwrap(); 
            if difficulty > 0 && difficulty < 4 {
                setlatch = false;
            }
        }
        tais.push(super::AI::BuildAIinfo(count as i32, difficulty));
        count += 1;
    }
    Game {
        fulldeck: tDealDeck,
        player: {super::Player::BuildPlayerID(1)},
        aiplayers: tais,
    }
}

/* First Deal of the game sets up a hand for each player and ai */
pub fn InitialDeal(game: &mut Game) {
    let mut count = 0;
    while count < 2 { //deals 2 cards to each player and ai
        let mut rng = rand::thread_rng();
        game.player.hand.push(game.fulldeck.remove(rng.gen_range(0..(game.fulldeck.len() as i32) as usize)));
        for ai in &mut game.aiplayers {
            ai.hand.push(game.fulldeck.remove(rng.gen_range(0..(game.fulldeck.len() as i32) as usize)));
        }
        count += 1;
    }
}

pub fn GetWinner(aiplayers: &Vec<AI>, player: &Player, round: &mut bool) {
    let mut wintie: Vec<AI> = vec![];
    let mut max = 0;
    for ai in aiplayers {
        if (ai.TotalHand() >= max) && (ai.TotalHand() <= 21) {
            max = ai.TotalHand();
        }
    }
    for ai in aiplayers {
        if ai.TotalHand() == max {
            println!("max: {}", max);
            wintie.push(ai.clone());
        }
    } //Player check still needed
    if wintie.len() > 1 {
        *round = true;
        let mut playerids = "".to_string();
        for player in &wintie {
            playerids.push_str(player.id.to_string().as_str());
            playerids.push_str(" ,");
        }
        println!("There is a tie. Players {} will have to player another round to determine a winner", playerids);
    } else {
        *round = false;
        println!("Player {} is the Winner!", wintie[0].id);
    }
}

pub fn DisplayGameState(aiplayers: &Vec<AI>, curr: &Player) {
    curr.DisplayHand();
    for ai in aiplayers {
        print!("AI {} hand: ", ai.id);
        for card in &ai.hand {
            if card == &ai.hand[0] {
                print!("'hidden' ");
                continue;
            }
            print!("{}, ", card.name_);
        }
        print!("\n");
    }
}

impl Game {
    pub fn cloneAIPlayers(&self) -> Vec<AI> {
        let mut outvec: Vec<AI> = vec![];
        for ai in &self.aiplayers {
            outvec.push(ai.clone())
        }
        return outvec;
    }
    pub fn cloneDeck(&self) -> Vec<Card> {
        let mut out: Vec<Card> = vec![];
        for card in &self.fulldeck {
            out.push(card.clone());
        }
        return out;
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
pub fn GetAIs(game: Game) -> u8 {
    return game.aiplayers.len() as u8;
}
