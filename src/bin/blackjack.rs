use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() {
    println!("Welcome to Blackjack!");
    let mut balance = 100;
    loop {
        println!("Your balance: ${}", balance);
        if balance == 0 {
            println!("You're out of money. Game over!");
            break;
        }

        println!("Place your bet (or 'q' to quit):");
        let mut bet_input = String::new();
        io::stdin().read_line(&mut bet_input).expect("Failed to read line");
        if bet_input.trim() == "q" {
            break;
        }
        let mut bet: u32 = bet_input.trim().parse().expect("Invalid bet amount");
        if bet > balance {
            println!("Insufficient balance. Please enter a lower bet.");
            continue;
        }

        play_blackjack(&mut balance, &mut bet);
    }
}

fn play_blackjack(balance: &mut u32, bet: &mut u32) {
    let mut deck = create_deck();
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    let mut player_hands = vec![vec![deck.pop().unwrap(), deck.pop().unwrap()]];
    let mut dealer_hand = vec![deck.pop().unwrap(), deck.pop().unwrap()];

    println!("Your hand: {:?}, your score: {:?}", player_hands[0], hand_value(&player_hands[0]));
    println!("Dealer shows: {:?}", dealer_hand[0]);

    let mut current_hand_index = 0;
    while current_hand_index < player_hands.len() {
        let mut player_turn = true;
        while player_turn {
            println!("Hand {}: {:?}, your score: {:?}", current_hand_index + 1, player_hands[current_hand_index], hand_value(&player_hands[current_hand_index]));
            println!("Do you want to (h)it, (s)tand, (d)ouble down, or (p)lit?");
            let mut action = String::new();
            io::stdin().read_line(&mut action).expect("Failed to read line");
            match action.trim() {
                "h" => {
                    player_hands[current_hand_index].push(deck.pop().unwrap());
                    println!("You hit.");
                    if hand_value(&player_hands[current_hand_index]) > 21 {
                        println!("Bust! You lose this hand.");
                        player_turn = false;
                    }
                }
                "s" => {
                    println!("You stand.");
                    player_turn = false;
                    current_hand_index += 1;
                }
                "d" => {
                    if *balance < *bet * 2 {
                        println!("Insufficient balance to double down.");
                    } else {
                        player_hands[current_hand_index].push(deck.pop().unwrap());
                        println!("You double down.");
                        if hand_value(&player_hands[current_hand_index]) > 21 {
                            println!("Bust! You lose this hand.");
                            *balance -= *bet * 2;
                            player_turn = false;
                            current_hand_index += 1;
                        } else {
                            player_turn = false;
                            current_hand_index += 1;
                            *bet *= 2;
                        }
                    }
                }
                "p" => {
                    if player_hands.len() == 1 && can_split(&player_hands[0]) {
                        let mut new_hand = vec![player_hands[0].pop().unwrap()];
                        new_hand.push(deck.pop().unwrap());
                        player_hands.push(new_hand);
                        player_hands[0].push(deck.pop().unwrap());
                        println!("You split your hand.");
                    } else {
                        println!("You cannot split your hand.");
                    }
                }
                _ => println!("Invalid input. Please enter 'h' to hit, 's' to stand, 'd' to double down, or 'p' to split."),
            }
        }
    }

    while hand_value(&dealer_hand) < 17 {
        dealer_hand.push(deck.pop().unwrap());
        println!("Dealer hits.");
        println!("Dealer's hand: {:?}", dealer_hand);
    }

    println!("Dealer's score: {}", hand_value(&dealer_hand));
    let mut player_wins = 0;
    let mut player_loses = 0;
    for hand in &player_hands {
        let player_score = hand_value(hand);
        println!("Your score: {}", player_score);
        if hand_value(&dealer_hand) > 21 || player_score > hand_value(&dealer_hand) {
            println!("You win this hand!");
            player_wins += 1;
        } else {
            println!("Dealer wins this hand.");
            player_loses += 1;
        }
    }

    if player_wins > 0 {
        println!("You won {} hand(s)!", player_wins);
        *balance += *bet * player_wins as u32;
    }
    if player_loses > 0 {
        println!("You lost {} hand(s).", player_loses);
        *balance -= *bet * player_loses as u32;
    }
}

fn can_split(hand: &[String]) -> bool {
    let rank = hand[0].split_whitespace().next().unwrap();
    hand.iter().all(|card| card.split_whitespace().next().unwrap() == rank)
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