
#[derive(PartialEq)]
pub struct Card {
    pub name_: String,
    pub value_: u8,
}

pub fn BuildSuits() -> Vec<String> {
    let mut suits: Vec<String> = vec![];
    suits.push("Clubs".to_string());
    suits.push("Spades".to_string());
    suits.push("Diamonds".to_string());
    suits.push("Hearts".to_string());
    return suits;
}
pub fn BuildNumber() -> Vec<String> {
    let mut number: Vec<String> = vec![];
    number.push("2".to_string());
    number.push("3".to_string());
    number.push("4".to_string());
    number.push("5".to_string());
    number.push("6".to_string());
    number.push("7".to_string());
    number.push("8".to_string());
    number.push("9".to_string());
    number.push("10".to_string());
    number.push("Jack".to_string());
    number.push("Queen".to_string());
    number.push("King".to_string());
    number.push("Ace".to_string());
    return number;
}

impl Card {
    pub fn clone(&self) -> Card {
        Card {
            name_: self.name_.clone(),
            value_: self.value_.clone(),
        }
    }
}

pub fn BuildCard(name: &str) -> Card {
    let suits: Vec<String> = BuildSuits();
    Card {
        name_: name.to_string(),
        value_: {
            let mut hold: String = name.to_string();
            for suit in suits {
                hold = hold.replace(suit.as_str(), "");
                hold = hold.replace(" of ", "");
            }
            match hold.as_str() {
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                "6" => 6,
                "7" => 7,
                "8" => 8,
                "9" => 9,
                "10" | "Jack" | "Queen" | "King" => 10,
                "Ace" => 11,
                _ => 0,
            }

        }
    }
}

pub fn BuildDeck() -> Vec<Card> {
    let suits: Vec<String> = BuildSuits();
    let number: Vec<String> = BuildNumber();
    let mut deck: Vec<Card> = vec![];
    for suit in &suits {
        for num in &number {
            let nsuit: &str = suit.as_str();
            let mut tag: String = num.clone();
            tag.push_str(" of ");
            tag.push_str(nsuit);
            let ncard: Card = BuildCard(tag.as_str());
            deck.push(ncard);
        }
    }
    return deck;
}
