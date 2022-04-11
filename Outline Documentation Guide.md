Cards {name, value}
    Card();
    Deck();
    Clone();
Player {hand, id}
    Player();
    Clone();
BlackJack 
    Game {players, dealdeck}

    Start Game
    Deal Hands
    Player hit/pass. Repeat until game over
    Display Table
    Win Check at End.

    BlackJackCondiditon checks
        Handtotal value <= 21; otherwise fail/ cant play rest of hand;
        if 6 >= cards in hand. Auto win. Endgame.

===== BASE COMPLETE (some clean up and effeicency left to fix) =====

AI 
    Difficulty settings
    Read & understand Hands
        Should be easier since this will propably run on TotalHand()
        and the probablility of cards left in the deck.
        Can be implemented differently but probablility was my first idea for easy but non-dynamic AI
    Chose to 'hit' or 'pass'

HouseHand
    Mainly build off of AI with certian limits and very good.
    Players against the house not eachother
    Will not hit on 16 or higher

Multiplayer
    Some UI or method of multipule people playing on a single terminal/comupter
    Im not totally sure how to do this yet so if you have ideas develop and outline them please.

Network Multiplayer
    Pretty straight foward Multiplayer on more than one terminal.
    Again I'm not sure how to do this so if you have ideas develop and outline them please.

UI - Graphics
    Developing interactive interface that is not the text editor in terminal we currently use.
    Iced or fltk might be good Rust UI framework to achieve this goal.
    UI outline:
        a home page to choose play offline or with other players :
            -a text entry to input player name
            -a button to offline game
            -a button to multiplayer game
        game page which is transferred from home page :
            -hit button in all cases
            -pass button to pass
    

        
