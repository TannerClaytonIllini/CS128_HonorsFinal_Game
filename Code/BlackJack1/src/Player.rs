use super::cards::*;

//Player Elements and Data. mainly for later neworking or turns

#[derive(PartialEq)]
pub struct Player {
    pub hand: Vec<Card>,
    pub id: i32,
}

impl Player {
    pub fn clone(&self) -> Player {
        let mut chand: Vec<Card> = vec![];
        for card in &self.hand {
            chand.push(card.clone());
        }
        Player {
            hand: chand,
            id: self.id.clone(),
        }
    }
}

/* creates new player */
pub fn BuildPlayer() -> Player{
    Player {
        hand: vec![],
        id: 0,
    }
}
/* creates new player with specified 'count'/integer id */
pub fn BuildPlayerID(id: i32) -> Player{
    Player {
        hand: vec![],
        id: id,
    }
}