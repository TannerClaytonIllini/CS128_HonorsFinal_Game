use super::cards::*;

//Player Elements and Data. mainly for later neworking or turns

pub struct Player {
    pub hand: Vec<Card>,
}

pub fn BuildPlayer() -> Player{
    Player {
        hand: vec![],
    }
}