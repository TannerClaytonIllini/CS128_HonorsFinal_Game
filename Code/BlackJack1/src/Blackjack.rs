use super::cards::*;
use super::Player::*;

pub struct Game {
    DealDeck: Vec<Card>,
    players: Vec<Player>,
}

pub fn main() {
    let gamestate: Game = Setup(3, 3);
    println!("BlackJack main");
}

pub fn Setup(players: u8, num: u8) -> Game {
    let mut tDealDeck: Vec<Card> = vec![];
    let mut count = 1;
    while count <= num {
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

pub fn GetPlayers() -> u8 {
    return 1;
}

pub fn DealHands(gamestate: Game, players: Vec<Player>) {

}