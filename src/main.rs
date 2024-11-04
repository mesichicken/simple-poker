use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

impl Card {
    fn new(suit: Suit, rank: i32) -> Self {
        Card { suit, rank }
    }
}

fn build_deck() -> Vec<Card> {
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
    let mut deck = Vec::with_capacity(52);

    for &suit in &suits {
        for rank in 1..=13 {
            deck.push(Card::new(suit, rank));
        }
    }

    deck
}

fn draw_hand(deck: &mut Vec<Card>, num_cards: usize) -> Vec<Card> {
    (0..num_cards).filter_map(|_| deck.pop()).collect()
}

fn print_hand(hand: &[Card], title: &str) {
    println!("---{}---", title);
    for (i, card) in hand.iter().enumerate() {
        println!("{}: {:?} {}", i + 1, card.suit, card.rank);
    }
}

fn replace_cards(hand: &mut Vec<Card>, deck: &mut Vec<Card>, indices: &[usize]) {
    for &index in indices {
        if let Some(card) = deck.pop() {
            hand[index - 1] = card;
        }
    }
    hand.sort_by_key(|card| card.rank);
}

fn check_flush(hand: &[Card]) -> bool {
    let suit = hand.first().unwrap().suit;
    hand.iter().all(|card| card.suit == suit)
}

fn check_pairs(hand: &[Card]) -> HashMap<i32, usize> {
    let mut rank_count = HashMap::new();
    for card in hand {
        *rank_count.entry(card.rank).or_insert(0) += 1;
    }
    rank_count
}

fn evaluate_hand(hand: &[Card]) {
    if check_flush(hand) {
        println!("フラッシュ!");
        return;
    }

    let rank_count = check_pairs(hand);
    let pair_counts: Vec<usize> = rank_count.values().cloned().collect();

    if pair_counts.contains(&3) {
        println!("スリーカード!");
    } else if pair_counts.iter().filter(|&&x| x == 2).count() == 2 {
        println!("2ペア!");
    } else if pair_counts.contains(&2) {
        println!("1ペア!");
    } else {
        println!("役なし...");
    }
}

fn main() {
    let mut deck = build_deck();

    // デッキをシャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    // 5枚の初期手札を引く
    let mut hand = draw_hand(&mut deck, 5);
    hand.sort_by_key(|card| card.rank);

    print_hand(&hand, "Initial Hand");

    // 交換するカードの番号を取得
    println!("交換したいカードの番号を入力してください (例: 1 2 3):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力の読み取りに失敗しました");

    let indices: Vec<usize> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    // 選択したカードを交換
    replace_cards(&mut hand, &mut deck, &indices);

    print_hand(&hand, "Final Hand");

    // 手札の評価
    evaluate_hand(&hand);
}
