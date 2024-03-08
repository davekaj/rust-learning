use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() {
    println!("Welcome to Blackjack!");
    play_blackjack();
}

fn play_blackjack() {
    let mut deck = create_deck();
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    let mut player_hand = vec![deck.pop().unwrap(), deck.pop().unwrap()];
    let mut dealer_hand = vec![deck.pop().unwrap(), deck.pop().unwrap()];

    println!("Your hand: {:?}, your score: {:?}", player_hand, hand_value(&player_hand));
    println!("Dealer shows: {:?}", dealer_hand[0]);

    loop {
        println!("Do you want to (h)it or (s)tand?");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read line");
        match action.trim() {
            "h" => {
                player_hand.push(deck.pop().unwrap());
                println!("You hit.");
                println!("Your hand: {:?}, your score: {:?}", player_hand, hand_value(&player_hand));
                if hand_value(&player_hand) > 21 {
                    println!("Bust! You lose.");
                    return;
                }
            }
            "s" => {
                println!("You stand.");
                break;
            }
            _ => println!("Invalid input. Please enter 'h' to hit or 's' to stand."),
        }
    }

    while hand_value(&dealer_hand) < 17 {
        dealer_hand.push(deck.pop().unwrap());
        println!("Dealer hits. New score: {}", hand_value(&dealer_hand));
        println!("Dealer's hand: {:?}", dealer_hand);
    }

    println!("Dealer's score: {}", hand_value(&dealer_hand));
    println!("Your score: {}", hand_value(&player_hand));

    if hand_value(&dealer_hand) > 21 || hand_value(&player_hand) > hand_value(&dealer_hand) {
        println!("You win!");
    } else {
        println!("Dealer wins.");
    }
}

fn create_deck() -> Vec<String> {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];
    let mut deck = Vec::new();
    for &suit in &suits {
        for &rank in &ranks {
            deck.push(format!("{} of {}", rank, suit)); // Directly push owned String
        }
    }
    deck
}

fn hand_value(hand: &[String]) -> i32 {
    let mut value = 0;
    let mut has_ace = false;
    for card in hand {
        let rank = card.split_whitespace().next().unwrap();
        let card_value = match rank {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" | "Jack" | "Queen" | "King" => 10,
            "Ace" => {
                has_ace = true;
                11
            }
            _ => 0,
        };
        value += card_value;
    }

    if has_ace && value > 21 {
        let mut new_value = value;
        for card in hand {
            let rank = card.split_whitespace().next().unwrap();
            if rank == "Ace" {
                new_value -= 10;
                println!("Ace counted as 1");
                break;
            }
        }
        new_value
    } else {
        value
    }
}