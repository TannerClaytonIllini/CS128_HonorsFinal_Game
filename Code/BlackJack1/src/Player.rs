use super::cards::*;

//Player Elements and Data. mainly for later neworking or turns

pub struct Player {
    hand: Vec<Card>,
}

pub fn BuildPlayer() -> Player{
    Player {
        hand: vec![],
    }
}