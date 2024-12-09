use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct CardDeck {
    cards: Vec<String>,
}

impl CardDeck {
    fn new() -> Self {
        // create a not list, but an array of suits: 'spades', 'diamonds', ...
        // we want to use array, because arrays have constant length
        let suits = ["spades", "diamonds", "hearts", "clubs"];

        // create a list of values: 'two', 'three' ... 'ace'
        let values = ["two", "tree", "four", "ace"];

        let mut cards = vec![];

        // with a doubles loop
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // let card_deck = CardDeck { cards };
        // return card_deck;

        // return CardDeck { cards };

        CardDeck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, quantity_of_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - quantity_of_cards)
    }
}

fn main() {
    let mut card_deck = CardDeck::new();

    card_deck.shuffle();

    // TODO: realize error handling with the next function. We want to be sure, that put in quantity is less then all cards.
    let dale_cards = card_deck.deal(3);

    println!("Card deck is: {:#?}", card_deck);
    println!("Dale cards (player's hand): {:#?}", dale_cards);
}
