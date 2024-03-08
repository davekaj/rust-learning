use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

#[derive(Debug, Clone)]
struct Card {
    rank: &'static str,
    suit: &'static str,
}

impl Card {
    fn value(&self) -> Vec<i32> {
        match self.rank {
            "Ace" => vec![1, 11],
            "2" => vec![2],
            "3" => vec![3],
            "4" => vec![4],
            "5" => vec![5],
            "6" => vec![6],
            "7" => vec![7],
            "8" => vec![8],
            "9" => vec![9],
            _ => vec![10], // Covers 10, Jack, Queen, King
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn values(&self) -> Vec<i32> {
        let mut possible_values = vec![0];
        for card in &self.cards {
            let mut new_values = Vec::new();
            for prev_value in &possible_values {
                for card_value in card.value() {
                    new_values.push(prev_value + card_value);
                }
            }
            possible_values = new_values;
        }
        possible_values.sort();
        possible_values.dedup();
        possible_values
    }

    fn best_value(&self) -> i32 {
        self.values()
            .into_iter()
            .filter(|&v| v <= 21)
            .max()
            .unwrap_or_else(|| *self.values().iter().min().unwrap())
    }
}

fn create_deck() -> Vec<Card> {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = [
        "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
    ];

    let mut deck = Vec::new();
    for &suit in &suits {
        for &rank in &ranks {
            deck.push(Card { rank, suit });
        }
    }
    deck
}

fn play_blackjack() {
    let mut deck = create_deck();
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    let mut player_hand = Hand::new();
    player_hand.add_card(deck.pop().unwrap());
    player_hand.add_card(deck.pop().unwrap());

    let mut dealer_hand = Hand::new();
    dealer_hand.add_card(deck.pop().unwrap());
    dealer_hand.add_card(deck.pop().unwrap());

    // Gameplay logic including hit or stand for player
    // Dealer's turn
    // Determining the outcome

    // This is a conceptual placeholder. You would need to implement the logic for:
    // 1. Player decisions (hit/stand)
    // 2. Dealer play according to Blackjack rules
    // 3. Determine and announce the winner
}

fn main() {
    println!("Welcome to Blackjack!");
    play_blackjack();
}
