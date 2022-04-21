pub mod cards;
pub mod Blackjack;
pub mod BlackjackAI;
pub mod Player;
pub mod AI;
pub mod menu;
use crate::cards::*;
use crate::Blackjack::*;
use crate::BlackjackAI::*;
use crate::Player::*;
use crate::AI::*;
use crate::menu::*;

fn main() {
    menu::main();
}
